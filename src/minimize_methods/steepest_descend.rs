use super::MinimizeMethod;

pub struct SteepestDescend {
}

impl SteepestDescend {
    pub fn new() -> SteepestDescend {
        SteepestDescend {
        }
    }
}

impl MinimizeMethod for SteepestDescend {
    fn step(&mut self, coord: (f64, f64), f: &mut dyn FnMut(f64, f64) -> f64, f_deriv: &mut dyn FnMut(f64, f64) -> (f64, f64)) -> (f64, f64) {
        let step = 0.1;
        let deriv = f_deriv(coord.0, coord.1);
        let mut new_coord = (coord.0 - deriv.0 * step, coord.1 - deriv.1 * step);
        new_coord
    }
}
