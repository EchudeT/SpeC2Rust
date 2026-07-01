use std::sync::Once;

#[derive(Debug, Default, Clone, Copy)]
pub struct MsvcInval;

impl MsvcInval {
    pub fn ensure_handler() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {});
    }
}
