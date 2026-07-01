use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::rc::Rc;

type HookId = usize;

type ClosePrimary = dyn FnMut(i32) -> io::Result<i32>;
type IoctlPrimary<T> = dyn FnMut(i32, i32, &mut T) -> io::Result<i32>;

type CloseHook =
    dyn FnMut(&mut dyn FnMut(i32) -> io::Result<i32>, i32) -> io::Result<i32>;
type IoctlHook<T> =
    dyn FnMut(&mut dyn FnMut(i32, i32, &mut T) -> io::Result<i32>, i32, i32, &mut T) -> io::Result<i32>;

thread_local! {
    static REGISTRY: RefCell<Vec<RegistryEntry>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: RefCell<HookId> = const { RefCell::new(1) };
}


struct RegistryEntry {
    id: HookId,
    close: Rc<RefCell<Box<CloseHook>>>,
}

pub struct FdHook<T = ()> {
    id: HookId,
    close_hook: Rc<RefCell<Box<CloseHook>>>,
    ioctl_hook: Rc<RefCell<Box<IoctlHook<T>>>>,
}

impl<T> FdHook<T> {
    pub fn new<
        C: FnMut(&mut dyn FnMut(i32) -> io::Result<i32>, i32) -> io::Result<i32> + 'static,
        I: FnMut(&mut dyn FnMut(i32, i32, &mut T) -> io::Result<i32>, i32, i32, &mut T) -> io::Result<i32>
            + 'static,
    >(
        close_hook: Option<C>,
        ioctl_hook: Option<I>,
    ) -> Self {
        let id = NEXT_ID.with(|next_id| {
            let mut next_id = next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        });

        let close_impl: Box<CloseHook> = match close_hook {
            Some(hook) => Box::new(hook),
            None => Box::new(|next, fd| next(fd)),
        };

        let ioctl_impl: Box<IoctlHook<T>> = match ioctl_hook {
            Some(hook) => Box::new(hook),
            None => Box::new(|next, fd, request, arg| next(fd, request, arg)),
        };

        Self {
            id,
            close_hook: Rc::new(RefCell::new(close_impl)),
            ioctl_hook: Rc::new(RefCell::new(ioctl_impl)),
        }
    }

    pub fn register(&self) {
        REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();

            if let Some(existing) = registry.iter().find(|entry| entry.id == self.id) {
                if !Rc::ptr_eq(&existing.close, &self.close_hook) {
                    panic!("fd hook link is already in use with different hook behavior");
                }
                return;
            }

            registry.insert(
                0,
                RegistryEntry {
                    id: self.id,
                    close: Rc::clone(&self.close_hook),
                },
            );
        });
    }

    pub fn unregister(&self) {
        REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            if let Some(pos) = registry.iter().position(|entry| entry.id == self.id) {
                registry.remove(pos);
            }
        });
    }

    pub fn execute_close_chain(
        primary: impl FnMut(i32) -> io::Result<i32>,
        fd: i32,
    ) -> io::Result<i32> {
        let entries = REGISTRY.with(|registry| {
            registry
                .borrow()
                .iter()
                .map(|entry| Rc::clone(&entry.close))
                .collect::<Vec<_>>()
        });

        Self::run_close_entries(&entries, 0, &mut primary.into_primary(), fd)
    }

    pub fn execute_ioctl_chain(
        hooks: &[&FdHook<T>],
        primary: impl FnMut(i32, i32, &mut T) -> io::Result<i32>,
        fd: i32,
        request: i32,
        arg: &mut T,
    ) -> io::Result<i32> {
        let mut seen = HashSet::new();
        let entries = hooks
            .iter()
            .filter(|hook| seen.insert(hook.id))
            .map(|hook| Rc::clone(&hook.ioctl_hook))
            .collect::<Vec<_>>();

        Self::run_ioctl_entries(&entries, 0, &mut primary.into_ioctl_primary(), fd, request, arg)
    }

    fn run_close_entries(
        entries: &[Rc<RefCell<Box<CloseHook>>>],
        index: usize,
        primary: &mut ClosePrimary,
        fd: i32,
    ) -> io::Result<i32> {
        if index >= entries.len() {
            primary(fd)
        } else {
            let entry = Rc::clone(&entries[index]);
            let mut next = |fd| Self::run_close_entries(entries, index + 1, primary, fd);
            let mut hook = entry.borrow_mut();
            (*hook)(&mut next, fd)
        }
    }

    fn run_ioctl_entries(
        entries: &[Rc<RefCell<Box<IoctlHook<T>>>>],
        index: usize,
        primary: &mut IoctlPrimary<T>,
        fd: i32,
        request: i32,
        arg: &mut T,
    ) -> io::Result<i32> {
        if index >= entries.len() {
            primary(fd, request, arg)
        } else {
            let entry = Rc::clone(&entries[index]);
            let mut next =
                |fd, request, arg| Self::run_ioctl_entries(entries, index + 1, primary, fd, request, arg);
            let mut hook = entry.borrow_mut();
            (*hook)(&mut next, fd, request, arg)
        }
    }
}

impl FdHook<()> {
    pub fn new_passthrough() -> Self {
        Self::new(
            Some(|next, fd| next(fd)),
            Some(|next, fd, request, arg| next(fd, request, arg)),
        )
    }
}

impl<T> Drop for FdHook<T> {
    fn drop(&mut self) {
        self.unregister();
    }
}

trait IntoPrimary {
    fn into_primary(self) -> Box<ClosePrimary>;
}

impl<F> IntoPrimary for F
where
    F: FnMut(i32) -> io::Result<i32> + 'static,
{
    fn into_primary(self) -> Box<ClosePrimary> {
        Box::new(self)
    }
}

trait IntoIoctlPrimary<T> {
    fn into_ioctl_primary(self) -> Box<IoctlPrimary<T>>;
}

impl<T, F> IntoIoctlPrimary<T> for F
where
    F: FnMut(i32, i32, &mut T) -> io::Result<i32> + 'static,
{
    fn into_ioctl_primary(self) -> Box<IoctlPrimary<T>> {
        Box::new(self)
    }
}
