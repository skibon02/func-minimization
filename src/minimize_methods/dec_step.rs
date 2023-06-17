use super::MinimizeMethod;

pub struct DecStep {
    step: f64,
    step_num: u64
}

impl DecStep {
    pub fn new(initial_step: f64) -> DecStep {
        DecStep {
            step: initial_step,
            step_num: 1
        }
    }
}

impl MinimizeMethod for DecStep {
    fn step(&mut self, coord: (f64, f64), f: &mut dyn FnMut(f64, f64) -> f64, deriv: &mut dyn FnMut(f64, f64) -> (f64, f64)) -> (f64, f64) {
        let deriv = deriv(coord.0, coord.1);
        let step = self.step / self.step_num as f64;
        let res = (coord.0 - deriv.0 * step, coord.1 - deriv.1 * step);
        self.step_num += 1;
        res
    }
}
