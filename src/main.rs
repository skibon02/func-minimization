
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use minimize_methods::{MinimizableFunc, MinimizeWorker};
use minimize_methods::{const_step::ConstStep, dec_step::DecStep, split_step::SplitStep, steepest_descend::SteepestDescend};

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
    let mut alg = SteepestDescend::new();
    let mut algo = MinimizeWorker::new(f, &mut alg).with_cnt(1000);
    let mut total_f_calls = 0;
    while let Some(info) = algo.run_step() {
        println!("{info:?}");
        total_f_calls += info.calc_metric;
    }
    println!("Total function calls: {}", total_f_calls);

    println!("Program finished!")
}
