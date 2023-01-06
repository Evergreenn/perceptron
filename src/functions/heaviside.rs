pub trait Heaviside {
    fn heaviside(&self) -> i8;
}

impl Heaviside for f64 {
    fn heaviside(&self) -> i8 {
        (*self >= 0.0) as i8
    }
}
