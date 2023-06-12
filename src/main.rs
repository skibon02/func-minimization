
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use minimize_methods::{MinimizableFunc, MinimizeAlgo};

#[derive(Debug)]
struct F (f64);

impl MinimizableFunc for F {
    fn deriv(&self, x: f64, y: f64) -> (f64, f64) {
        let dx = -4.0*x*(y-x*x) + 2.0 * self.0 * (x - 1.0);
        let dy = 2.0 * (y - x * x);
        (dx, dy)
    }
}

fn main() {
    let f = F(0.1);
    let mut algo = MinimizeAlgo::new(f).with_cnt(40000);
    while let Some(info) = algo.run_step() {
        println!("{info:?}");
    }

    println!("Program finished!")
}