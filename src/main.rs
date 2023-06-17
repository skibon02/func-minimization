
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use minimize_methods::{MinimizableFunc, MinimizeWorker};
use minimize_methods::{const_step::ConstStep, dec_step::DecStep, split_step::SplitStep};

#[derive(Debug)]
struct F (f64);

impl MinimizableFunc for F {
    fn deriv(&self, x: f64, y: f64) -> (f64, f64) {
        let dx = -4.0*x*(y-x*x) + 2.0 * self.0 * (x - 1.0);
        let dy = 2.0 * (y - x * x);
        (dx, dy)
    }
    fn calc(&self, x:f64, y: f64) -> f64 {
        let p1 = y - x * x;
        let p2 = x - 1.0;
        p1 * p1 + self.0 * p2 * p2

    }
}

fn main() {
    let f = F(0.1);
    let mut alg = SplitStep::new(1.0);
    let mut algo = MinimizeWorker::new(f, &mut alg).with_cnt(100);
    while let Some(info) = algo.run_step() {
        println!("{info:?}");
    }

    println!("Program finished!")
}
