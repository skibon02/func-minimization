use super::MinimizeMethod;

pub struct ConstStep {
    pub step: f64
}

impl ConstStep {
    pub fn new(initial_step: f64) -> ConstStep {
        ConstStep {
            step: initial_step,
        }
    }
}


impl MinimizeMethod for ConstStep {

    fn step(&mut self, coord: (f64, f64), f: &mut dyn FnMut(f64, f64) -> f64, deriv: &mut dyn FnMut(f64, f64) -> (f64, f64)) -> (f64, f64) {
        let deriv = deriv(coord.0, coord.1);
        (coord.0 - deriv.0 * self.step, coord.1 - deriv.1 * self.step)
    }
}
