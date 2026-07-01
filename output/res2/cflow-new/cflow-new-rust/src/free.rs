pub struct Free;

impl Free {
    pub fn rpl_free<T>(value: Option<T>) {
        drop(value);
    }
}
