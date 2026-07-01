use std::cell::RefCell;
use std::io;

type CloseHook = Box<dyn Fn(i32) -> io::Result<()> + 'static>;
type IoctlHook = Box<dyn Fn(i32, i32, &[u8]) -> io::Result<()> + 'static>;

#[derive(Default)]
struct HookEntry {
    close_hook: Option<CloseHook>,
    ioctl_hook: Option<IoctlHook>,
}

#[derive(Default)]
struct HookRegistry {
    next_id: usize,
    entries: Vec<Option<HookEntry>>,
}

impl HookRegistry {
    fn insert(&mut self, entry: HookEntry) -> usize {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        if id >= self.entries.len() {
            self.entries.push(Some(entry));
        } else {
            self.entries[id] = Some(entry);
        }

        id
    }

    fn remove(&mut self, id: usize) {
        if let Some(slot) = self.entries.get_mut(id) {
            *slot = None;
        }
    }

    fn take_close_hooks(&mut self) -> Vec<CloseHook> {
        self.entries
            .iter_mut()
            .filter_map(|entry| entry.as_mut()?.close_hook.take())
            .collect()
    }

    fn take_ioctl_hooks(&mut self) -> Vec<IoctlHook> {
        self.entries
            .iter_mut()
            .filter_map(|entry| entry.as_mut()?.ioctl_hook.take())
            .collect()
    }

    fn restore_close_hooks(&mut self, hooks: Vec<CloseHook>) {
        let mut hooks = hooks.into_iter();
        for entry in self.entries.iter_mut().filter_map(Option::as_mut) {
            if entry.close_hook.is_none() {
                if let Some(hook) = hooks.next() {
                    entry.close_hook = Some(hook);
                } else {
                    break;
                }
            }
        }
    }

    fn restore_ioctl_hooks(&mut self, hooks: Vec<IoctlHook>) {
        let mut hooks = hooks.into_iter();
        for entry in self.entries.iter_mut().filter_map(Option::as_mut) {
            if entry.ioctl_hook.is_none() {
                if let Some(hook) = hooks.next() {
                    entry.ioctl_hook = Some(hook);
                } else {
                    break;
                }
            }
        }
    }
}

thread_local! {
    static REGISTRY: RefCell<HookRegistry> = RefCell::new(HookRegistry::default());
}

pub struct FdHook {
    id: usize,
}

impl FdHook {
    pub fn register<CF, IF>(close_hook: Option<CF>, ioctl_hook: Option<IF>) -> Self
    where
        CF: Fn(i32) -> io::Result<()> + 'static,
        IF: Fn(i32, i32, &[u8]) -> io::Result<()> + 'static,
    {
        let entry = HookEntry {
            close_hook: close_hook.map(|f| Box::new(f) as CloseHook),
            ioctl_hook: ioctl_hook.map(|f| Box::new(f) as IoctlHook),
        };

        let id = REGISTRY.with(|registry| registry.borrow_mut().insert(entry));
        Self { id }
    }

    pub fn run_close_hooks(
        fd: i32,
        primary: impl FnOnce(i32) -> io::Result<()>,
    ) -> io::Result<()> {
        let hooks = REGISTRY.with(|registry| registry.borrow_mut().take_close_hooks());

        let primary_result = primary(fd);

        let mut first_hook_error = None;
        for hook in hooks {
            if let Err(err) = hook(fd) {
                if first_hook_error.is_none() {
                    first_hook_error = Some(err);
                }
            }
        }

        REGISTRY.with(|registry| registry.borrow_mut().restore_close_hooks(Vec::new()));

        match primary_result {
            Err(err) => Err(err),
            Ok(()) => match first_hook_error {
                Some(err) => Err(err),
                None => Ok(()),
            },
        }
    }

    pub fn run_ioctl_hooks(
        fd: i32,
        request: i32,
        arg: &[u8],
        primary: impl FnOnce(i32, i32, &[u8]) -> io::Result<()>,
    ) -> io::Result<()> {
        let hooks = REGISTRY.with(|registry| registry.borrow_mut().take_ioctl_hooks());

        let primary_result = primary(fd, request, arg);

        let mut first_hook_error = None;
        for hook in hooks {
            if let Err(err) = hook(fd, request, arg) {
                if first_hook_error.is_none() {
                    first_hook_error = Some(err);
                }
            }
        }

        REGISTRY.with(|registry| registry.borrow_mut().restore_ioctl_hooks(Vec::new()));

        match primary_result {
            Err(err) => Err(err),
            Ok(()) => match first_hook_error {
                Some(err) => Err(err),
                None => Ok(()),
            },
        }
    }

    pub fn unregister(self) {
        REGISTRY.with(|registry| registry.borrow_mut().remove(self.id));
    }
}

impl Drop for FdHook {
    fn drop(&mut self) {
        REGISTRY.with(|registry| registry.borrow_mut().remove(self.id));
    }
}
