
pub trait MinimizableFunc {
    fn deriv(&self, x: f64, y: f64) -> (f64, f64);
}

pub struct MinimizeAlgo<T: MinimizableFunc> {
    func: T,
    x: f64,
    y: f64,
    cnt: usize
}

#[derive(Debug)]
pub struct StepData {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    new_x: f64,
    new_y: f64
}

impl<T: MinimizableFunc> MinimizeAlgo<T> {
    pub fn new(f: T) -> MinimizeAlgo<T> {
        MinimizeAlgo { func: f, x: 0.0, y: 0.0, cnt: 10 }
    }
    pub fn with_start_point(self, x0: f64, y0: f64) -> MinimizeAlgo<T> {
        MinimizeAlgo { x: x0, y: y0, ..self }
    }
    pub fn with_cnt(self, cnt: usize) -> MinimizeAlgo<T> {
        MinimizeAlgo { cnt, ..self }
    }
    pub fn run_step(&mut self) -> Option<StepData> {
        if self.cnt == 0 {
            return None;
        }

        let (x,y) = (self.x, self.y);
        let (dx, dy) = self.func.deriv(self.x, self.y);

        

        
        //const step
        const step_size: f64 = 0.01;
        self.x -= step_size * dx;
        self.y -= step_size * dy;

        let (new_x, new_y) = (self.x, self.y);

        let step_data = StepData {
            x, y,
            dx, dy,
            new_x, new_y
        };


        self.cnt -= 1;
        Some(step_data)
    }
}