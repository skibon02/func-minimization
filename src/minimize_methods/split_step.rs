use super::MinimizeMethod;

pub struct SplitStep {
    step: f64,
}

impl SplitStep {
    pub fn new(initial_step: f64) -> SplitStep {
        SplitStep {
            step: initial_step,
        }
    }
}

impl MinimizeMethod for SplitStep {

    fn step(&mut self, coord: (f64, f64), deriv: (f64, f64)) -> (f64, f64) {
        let step = self.step;
        let res = (coord.0 - deriv.0 * step, coord.1 - deriv.1 * step);
        self.step /= 2.0;
        res
    }
}
