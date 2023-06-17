use super::MinimizeMethod;

pub struct ConstStep {
    pub step: f64
}

impl MinimizeMethod for ConstStep {

    fn step(&mut self, coord: (f64, f64), deriv: (f64, f64)) -> (f64, f64) {
        (coord.0 - deriv.0 * self.step, coord.1 - deriv.1 * self.step)
    }
}
