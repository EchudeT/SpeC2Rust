pub struct Itold;

impl Itold {
    pub fn qp_itoq(result: &mut f64, a: i32) {
        *result = (a as f64) as f64;
    }
}
