pub struct MsvcNothrow;

impl MsvcNothrow {
    pub fn get_osfhandle(fd: i32) -> Option<i64> {
        if fd < 0 {
            None
        } else {
            Some(i64::from(fd))
        }
    }
}
