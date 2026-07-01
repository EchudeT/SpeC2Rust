pub struct SetlocaleNullUnlocked;

impl SetlocaleNullUnlocked {
    pub fn query(category: i32) -> Option<String> {
        crate::setlocale_null::SetlocaleNull::setlocale_null(category)
    }
}
