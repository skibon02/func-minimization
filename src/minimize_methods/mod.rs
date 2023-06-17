pub mod const_step;
pub mod dec_step;
pub mod split_step;

pub trait MinimizableFunc {
    fn deriv(&self, x: f64, y: f64) -> (f64, f64);
    fn calc(&self, x:f64, y: f64) -> f64;
}
pub trait MinimizeMethod {
    fn step(&mut self, coord: (f64, f64), deriv: (f64, f64)) -> (f64, f64);
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
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    new_x: f64,
    new_y: f64,
    f: f64
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
        let (dx, dy) = self.func.deriv(self.x, self.y);

        

        
        //const step
        (self.x, self.y) = self.method.step((x,y),(dx,dy));

        let (new_x, new_y) = (self.x, self.y);
        let val = self.func.calc(new_x, new_y);

        let step_data = StepData {
            x, y,
            dx, dy,
            new_x, new_y,
            f: val
        };


        self.cnt -= 1;
        Some(step_data)
    }
}
