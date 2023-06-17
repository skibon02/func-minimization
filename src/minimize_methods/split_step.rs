use super::MinimizeMethod;

pub struct SplitStep {
    step: f64,
    prev_func_res: Option<f64>
}

impl SplitStep {
    pub fn new(initial_step: f64) -> SplitStep {
        SplitStep {
            step: initial_step,
            prev_func_res: None
        }
    }
}

impl MinimizeMethod for SplitStep {
    fn step(&mut self, coord: (f64, f64), f: &mut dyn FnMut(f64, f64) -> f64, f_deriv: &mut dyn FnMut(f64, f64) -> (f64, f64)) -> (f64, f64) {
        if self.prev_func_res.is_none() {
            self.prev_func_res = Some(f(coord.0, coord.1));
        }
        let deriv = f_deriv(coord.0, coord.1);
        let step = self.step;
        let res = (coord.0 - deriv.0 * step, coord.1 - deriv.1 * step);
        self.step /= 2.0;
        res
    }
}
