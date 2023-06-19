pub mod const_step;
pub mod dec_step;
pub mod split_step;
pub mod steepest_descend;

pub trait MinimizableFunc {
    fn deriv(&self, x: f64, y: f64) -> (f64, f64);
    fn calc(&self, x:f64, y: f64) -> f64;
}
pub trait MinimizeMethod {
    fn step(&mut self, coord: (f64, f64), f: &mut dyn FnMut(f64, f64) -> f64, deriv: &mut dyn FnMut(f64, f64) -> (f64, f64)) -> (f64, f64);
}

pub struct MinimizeWorker<'a, T: MinimizableFunc> {
    func: T,
    x: f64,
    y: f64,
    cnt: usize,
    method: &'a mut dyn MinimizeMethod
}

#[derive(Debug)]
pub struct StepData {
    pub new_x: f64,
    pub new_y: f64,
    pub f: f64,
    pub calc_metric: u64,
    pub deriv_metric: u64
}

impl<'a, T: MinimizableFunc> MinimizeWorker<'a, T> {
    pub fn new(f: T, method: &'a mut dyn MinimizeMethod) -> MinimizeWorker<'a, T> {
        MinimizeWorker { func: f, x: 0.0, y: 0.0, cnt: 10, method }
    }
    pub fn with_start_point(self, x0: f64, y0: f64) -> MinimizeWorker<'a, T> {
        MinimizeWorker { x: x0, y: y0, ..self }
    }
    pub fn with_cnt(self, cnt: usize) -> MinimizeWorker<'a, T> {
        MinimizeWorker { cnt, ..self }
    }
    pub fn run_step(&mut self) -> Option<StepData> {
        if self.cnt == 0 {
            return None;
        }

        let (x,y) = (self.x, self.y);
        
        let mut calc_metric = 0;
        let mut deriv_metric = 0;
        //const step
        (self.x, self.y) = self.method.step((x,y), &mut |x, y| {
            calc_metric += 1;
            self.func.calc(x,y)
        }, &mut |x, y| {
            deriv_metric += 1;
            self.func.deriv(x,y) 
        });

        let (new_x, new_y) = (self.x, self.y);
        let val = self.func.calc(new_x, new_y);

        let step_data = StepData {
            new_x, new_y,
            f: val,
            calc_metric,
            deriv_metric
        };


        self.cnt -= 1;
        Some(step_data)
    }
}
