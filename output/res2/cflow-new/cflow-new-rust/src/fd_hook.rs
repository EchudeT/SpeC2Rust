/// A single hook-link entry in a file-descriptor hook chain.
///
/// This Rust rewrite models the original C link object as an owned node-like
/// handle that can store optional close and ioctl callbacks together with an
/// activity flag. The surrounding registry/dispatch logic belongs elsewhere;
/// this type only represents one registered position in such a chain.
pub struct FdHook {
    close_hook: Option<Box<dyn Fn(i32) -> bool + 'static>>,
    ioctl_hook: Option<Box<dyn Fn(i32, i32, &[u8]) -> bool + 'static>>,
    active: bool,
}

impl FdHook {
    /// Creates an empty, active hook slot.
    pub fn new() -> Self {
        Self {
            close_hook: None,
            ioctl_hook: None,
            active: true,
        }
    }

    /// Creates a hook slot with an optional close hook.
    pub fn with_close_hook<F>(close_hook: F) -> Self
    where
        F: Fn(i32) -> bool + 'static,
    {
        Self {
            close_hook: Some(Box::new(close_hook)),
            ioctl_hook: None,
            active: true,
        }
    }

    /// Creates a hook slot with an optional ioctl hook.
    pub fn with_ioctl_hook<F>(ioctl_hook: F) -> Self
    where
        F: Fn(i32, i32, &[u8]) -> bool + 'static,
    {
        Self {
            close_hook: None,
            ioctl_hook: Some(Box::new(ioctl_hook)),
            active: true,
        }
    }

    /// Creates a hook slot with both close and ioctl hooks.
    pub fn with_hooks<FClose, FIoctl>(close_hook: FClose, ioctl_hook: FIoctl) -> Self
    where
        FClose: Fn(i32) -> bool + 'static,
        FIoctl: Fn(i32, i32, &[u8]) -> bool + 'static,
    {
        Self {
            close_hook: Some(Box::new(close_hook)),
            ioctl_hook: Some(Box::new(ioctl_hook)),
            active: true,
        }
    }

    /// Replaces the close hook.
    pub fn set_close_hook<F>(&mut self, close_hook: F)
    where
        F: Fn(i32) -> bool + 'static,
    {
        self.close_hook = Some(Box::new(close_hook));
    }

    /// Replaces the ioctl hook.
    pub fn set_ioctl_hook<F>(&mut self, ioctl_hook: F)
    where
        F: Fn(i32, i32, &[u8]) -> bool + 'static,
    {
        self.ioctl_hook = Some(Box::new(ioctl_hook));
    }

    /// Removes the close hook.
    pub fn clear_close_hook(&mut self) {
        self.close_hook = None;
    }

    /// Removes the ioctl hook.
    pub fn clear_ioctl_hook(&mut self) {
        self.ioctl_hook = None;
    }

    /// Marks the hook slot as active.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Marks the hook slot as inactive.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Returns whether this hook slot is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns whether a close hook is registered.
    pub fn has_close_hook(&self) -> bool {
        self.close_hook.is_some()
    }

    /// Returns whether an ioctl hook is registered.
    pub fn has_ioctl_hook(&self) -> bool {
        self.ioctl_hook.is_some()
    }

    /// Executes the close hook if this slot is active and such a hook exists.
    ///
    /// Returns `Some(result)` when a hook was invoked, otherwise `None`.
    pub fn run_close(&self, fd: i32) -> Option<bool> {
        if !self.active {
            return None;
        }
        self.close_hook.as_ref().map(|hook| hook(fd))
    }

    /// Executes the ioctl hook if this slot is active and such a hook exists.
    ///
    /// Returns `Some(result)` when a hook was invoked, otherwise `None`.
    pub fn run_ioctl(&self, fd: i32, request: i32, arg: &[u8]) -> Option<bool> {
        if !self.active {
            return None;
        }
        self.ioctl_hook
            .as_ref()
            .map(|hook| hook(fd, request, arg))
    }
}

impl Default for FdHook {
    fn default() -> Self {
        Self::new()
    }
}
