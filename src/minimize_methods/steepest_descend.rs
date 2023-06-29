use std::{iter::successors, collections::{BTreeMap}, cmp::Ordering};

use super::MinimizeMethod;

pub struct Point(f64, f64);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == self.0 && self.1 == other.1
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 < other.0 {
            return Some(Ordering::Less);
        }
        if self.0 > other.0 {
            return Some(Ordering::Greater);
        }
        if self.1 < other.1 {
            return Some(Ordering::Less);
        }
        if self.1 > other.1 {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Point{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct SteepestDescend {
    hashed_vals: BTreeMap<Point, f64>,
    init_step: f64
}

impl SteepestDescend {
    pub fn new(init_step: f64) -> SteepestDescend {
        SteepestDescend {
            hashed_vals: BTreeMap::new(),
            init_step
        }
    }
}

impl MinimizeMethod for SteepestDescend {
    fn step(&mut self, coord: (f64, f64), f: &mut dyn FnMut(f64, f64) -> f64, f_deriv: &mut dyn FnMut(f64, f64) -> (f64, f64)) -> (f64, f64) {
        let mut f = |x1, x2| -> f64 {
            *self.hashed_vals.entry(Point(x1, x2)).or_insert_with(|| f(x1, x2))
        };
        let epsilon = 0.000001;
        let deriv = f_deriv(coord.0, coord.1);
        
        let mut step = self.init_step;
        let final_coord = successors(Some(coord), |prev_coord| {
            let mut step1 = (prev_coord.0 - step * deriv.0, prev_coord.1 - step * deriv.1);
            let step2 = (prev_coord.0 - step * 2.0 * deriv.0, prev_coord.1 - step * 2.0 * deriv.1);

            let f0 = f(prev_coord.0, prev_coord.1);
            let mut f1 = f(step1.0, step1.1);
            let mut f2 = f(step2.0, step2.1);
            while !(f1 <= f0 && f2 <= f1) {
                step /= 2.0;
                if (step1.0 - step2.0).abs() < epsilon && (step1.1 - step2.1).abs() < epsilon &&
                (prev_coord.0 - step1.0).abs() < epsilon && (prev_coord.1 - step1.1).abs() < epsilon {
                    return None;
                }
                
                step1 = (prev_coord.0 - step * deriv.0, prev_coord.1 - step * deriv.1);

                f2 = f1;
                f1 = f(step1.0, step1.1);
            }

            Some(step1)
        });
        
        final_coord.last().unwrap()
    }
}
