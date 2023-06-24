
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use minimize_methods::{MinimizableFunc, MinimizeWorker};
use minimize_methods::*;
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};

use std::io::{Write, stdin};
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
        writeln!(stdout, "Находимся в точке {:.4}, {:.4}. \t\tЗначение функции: {:.6}", info.new_x, info.new_y, info.f);
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
    let mut algo: Box<dyn MinimizeMethod>;

    let mut params = Params {
        a: 0.1,
        x1_0: 0.0,
        x2_0: 0.0,
        method: MethodEnum::ConstStep,
        print_every: 1,
        initial_step: 0.01,
    };
    let mut buffer = String::new();


    print_title(&mut stdout);
    while true {
        let mut buffer = String::new();
        print_params(&mut stdout, &params);

        let mut spec = ColorSpec::new();
        stdout.clear(&mut spec);

        println!("Введите номер параметра, который хотите изменить, или нажмите Enter для запуска алгоритма...");
        println!();

        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().len() == 0 {
            let mut f = F(params.a);
        
            algo = match params.method {
                MethodEnum::ConstStep => Box::new(const_step::ConstStep::new(params.initial_step)),
                MethodEnum::DecStep => Box::new(dec_step::DecStep::new(params.initial_step)),
                MethodEnum::SplitStep => Box::new(split_step::SplitStep::new(params.initial_step)),
                MethodEnum::SteepestDescend => Box::new(steepest_descend::SteepestDescend::new()),
                _ => todo!()
            };

            println!("Введите количество шагов");
            buffer.clear();
            stdin().read_line(&mut buffer).unwrap();

            let steps = match buffer.trim().parse::<u32>() {
                Ok(v) => {
                    v
                },
                Err(_) => {
                    continue;
                }
            };
            
            run(&f, &mut algo, &mut stdout, params.x1_0, params.x2_0, steps as u64);
        }
        else {

            match buffer.trim().parse::<u32>() {
                Ok(v) => {
                    buffer.clear();
                    match v {
                        1 => {
                            println!("Введите новое значение коэффициента");
                            stdin().read_line(&mut buffer).unwrap();
                            let v = buffer.trim().parse::<f64>().unwrap();
                            params.a = v;
                        },
                        2 => {
                            println!("Введите новое начальное значение Х1");
                            stdin().read_line(&mut buffer).unwrap();
                            let v = buffer.trim().parse::<f64>().unwrap();
                            params.x1_0 = v;
                        },
                        3 => {
                            println!("Введите новое начальное значение Х2");
                            stdin().read_line(&mut buffer).unwrap();
                            let v = buffer.trim().parse::<f64>().unwrap();
                            params.x2_0 = v;
                        },
                        4 => {
                            println!("Введите новую начальную длину шага");
                            stdin().read_line(&mut buffer).unwrap();
                            let v = buffer.trim().parse::<f64>().unwrap();
                            params.initial_step = v;
                        },
                        5 => {
                            println!("Выберите алгоритм минимизации:");
                            println!("1. {}", MethodEnum::ConstStep.desc());
                            println!("2. {}", MethodEnum::DecStep.desc());
                            println!("3. {}", MethodEnum::SplitStep.desc());
                            println!("4. {}", MethodEnum::SteepestDescend.desc());
                            stdin().read_line(&mut buffer).unwrap();
                            params.method = match buffer.trim().parse::<u32>().unwrap() {
                                1 => MethodEnum::ConstStep,
                                2 => MethodEnum::DecStep,
                                3 => MethodEnum::SplitStep,
                                4 => MethodEnum::SteepestDescend,
                                _ => todo!()
                            };
                        },
                        6 => {
                            println!("Печать каждые _ шагов:");
                            stdin().read_line(&mut buffer).unwrap();
                            let v = buffer.trim().parse::<u64>().unwrap();
                            params.print_every = v;
                        },
                        _ => {
                            continue;
                        }

                    }
                },
                Err(_) => {
                    continue;
                }
            }
        }

    }



}
