use libc::c_float;

pub(crate) trait Pow34 {
    fn abs_pow34(self) -> Self;
}

impl Pow34 for c_float {
    fn abs_pow34(self) -> Self {
        let a = self.abs();
        (a * a.sqrt()).sqrt()
    }
}
