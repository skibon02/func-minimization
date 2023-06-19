
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use minimize_methods::{MinimizableFunc, MinimizeWorker};
use minimize_methods::*;
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use term_table::{self, TableBuilder, TableStyle};

use Color::{Green, Red, Rgb, Yellow, Black, Blue, Ansi256, Cyan, Magenta, White};

use crate::minimize_methods::MinimizeMethod;

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

trait FastColors {
    fn bg(&mut self, spec: &mut ColorSpec, col: Color);
    fn fg(&mut self, spec: &mut ColorSpec, col: Color);
    fn intense(&mut self, spec: &mut ColorSpec, intense: bool);
    fn clear(&mut self, spec: &mut ColorSpec);
}

impl FastColors for StandardStream {
    fn bg(&mut self, spec: &mut ColorSpec, col: Color) {
        self.set_color(spec.set_bg(Some(col))).unwrap();
    }
    fn fg(&mut self, spec: &mut ColorSpec, col: Color) {
        self.set_color(spec.set_fg(Some(col))).unwrap();
    }
    fn intense(&mut self, spec: &mut ColorSpec, intense: bool) {
        self.set_color(spec.set_intense(intense)).unwrap();
    }
    fn clear(&mut self, spec: &mut ColorSpec) {
        spec.set_fg(None);
        spec.set_bg(None);
        self.set_color(&ColorSpec::new());
    }
}

fn run(f: &impl MinimizableFunc, algo: &mut Box<dyn MinimizeMethod>, stdout: &mut StandardStream, start_x: f64, start_y: f64, steps: u64) {
    let mut worker = MinimizeWorker::new(f, algo.as_mut()).with_cnt(steps as usize).with_start_point(start_x, start_y);
    let mut total_f_calls = 0;
    let mut total_deriv_calls = 0;
    
    let mut spec = ColorSpec::new();
    stdout.fg(&mut spec, Green);

    while let Some(info) = worker.run_step() {
        writeln!(stdout, "Находимся в точке {:.4}, {:.4}. Значение функции: {:.6}", info.new_x, info.new_y, info.f);
        total_f_calls += info.calc_metric;
        total_deriv_calls += info.deriv_metric;
    }
    println!();

    stdout.clear(&mut spec);
    stdout.fg(&mut spec, Blue);
    stdout.intense(&mut spec, true);
    print!("Функция f была вычислена \t\t");
    stdout.fg(&mut spec, Red);
    print!("{} раз", total_f_calls);
    
    stdout.clear(&mut spec);
    println!();
    stdout.fg(&mut spec, Blue);
    stdout.intense(&mut spec, true);

    print!("Производная функции f была вычислена \t");
    stdout.fg(&mut spec, Red);
    print!("{} раз", total_deriv_calls);

    stdout.clear(&mut spec);
    println!();
}
enum MethodEnum {
    ConstStep,
    DecStep,
    SplitStep,
    SteepestDescend,
}
impl MethodEnum {
    fn desc(&self) -> &str {
        match self {
            MethodEnum::ConstStep => "Метод градиентного спуска с постоянным шагом",
            MethodEnum::DecStep => "Метод градиентного спуска с убыванием шага",
            MethodEnum::SplitStep => "Метод градиентного спуска с дроблением шага",
            MethodEnum::SteepestDescend => "Метод наискорейшего спуска",
        }
    }
}
struct Params {
    a: f64,
    x1_0: f64,
    x2_0: f64,
    method: MethodEnum,
    print_every: u64,
    initial_step: f64,
}
fn clear_screen() {
    print!("{}[2J", 27 as char);
}
fn print_params(stdout: &mut StandardStream, params: &Params) {
    let params = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new("1. Коеффициент А:"),
                TableCell::new("2. Начальное значение X1: "),
                TableCell::new("3. Начальное значение X2: "),
                TableCell::new("4. Начальная длина шага: "),
                TableCell::new("5. Алгоритм минимизации: "),
                TableCell::new("6. Печатать каждые _ шагов: "),
            ]),
            Row::new(vec![
                TableCell::new(params.a),
                TableCell::new(params.x1_0),
                TableCell::new(params.x2_0),
                TableCell::new(params.initial_step),
                TableCell::new(params.method.desc()),
                TableCell::new(params.print_every),
            ])
        ]
    ).build();

    let mut spec = ColorSpec::new();
    stdout.fg(&mut spec, Blue);

    println!("{}", params.render());
}

fn print_title(stdout: &mut StandardStream) {
    let table = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new_with_alignment("Лабораторная работа №1. Методы минимизации функций.", 2, Alignment::Center)
            ]),
            Row::new(vec![
                TableCell::new("Дана функция: "),
                TableCell::new_with_alignment("F(x1, x2) = (x2-x1^2)^2+a*(x1-1)^2", 1, Alignment::Center)
            ]),
        ]
    ).build();



    let mut spec = ColorSpec::new();
    stdout.fg(&mut spec, Blue);
    stdout.intense(&mut spec, true);

    println!("{}", table.render());
}



fn main() {
    clear_screen();
    
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut params = Params {
        a: 0.1,
        x1_0: 0.0,
        x2_0: 0.0,
        method: MethodEnum::ConstStep,
        print_every: 1,
        initial_step: 0.01,
    };


    print_title(&mut stdout);

    print_params(&mut stdout, &params);

    let mut spec = ColorSpec::new();
    stdout.clear(&mut spec);

    println!("Введите номер параметра, который хотите изменить, или нажмите Enter для запуска алгоритма...");
    println!();



    let mut f = F(0.1);
    let mut start_x = 0.11;
    let mut start_y = 0.003;
    let mut steps = 10;
    let mut algo: Box<dyn MinimizeMethod>;

    algo = Box::new(split_step::SplitStep::new(0.1));
    run(&f, &mut algo, &mut stdout, start_x, start_y, steps);
}
