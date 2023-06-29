
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use minimize_methods::{MinimizableFunc, MinimizeWorker};
use minimize_methods::*;
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};

use std::io::stdin;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use term_table::{self, TableBuilder, TableStyle};

use Color::{Green, Red, Blue};

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
        self.set_color(spec.set_bg(Some(col)));
    }
    fn fg(&mut self, spec: &mut ColorSpec, col: Color) {
        self.set_color(spec.set_fg(Some(col)));
    }
    fn intense(&mut self, spec: &mut ColorSpec, intense: bool) {
        self.set_color(spec.set_intense(intense));
    }
    fn clear(&mut self, spec: &mut ColorSpec) {
        spec.set_fg(None);
        spec.set_bg(None);
        self.set_color(&ColorSpec::new());
    }
}

fn run(f: &impl MinimizableFunc, algo: &mut Box<dyn MinimizeMethod>, stdout: &mut StandardStream, start_x: f64, start_y: f64, steps: u64, print_every: u64) {
    let mut worker = MinimizeWorker::new(f, algo.as_mut()).with_cnt(steps as usize).with_start_point(start_x, start_y);
    let mut total_f_calls = 0;
    let mut total_deriv_calls = 0;
    
    let mut spec = ColorSpec::new();

    let mut counter = 0;
    while let Some(info) = worker.run_step() {
        total_f_calls += info.calc_metric;
        total_deriv_calls += info.deriv_metric;

        if counter % print_every != 0 {
            counter += 1;
            continue;
        }


        stdout.intense(&mut spec, true);
        stdout.fg(&mut spec, Color::Red);
        print!("{}. ", counter);
        stdout.intense(&mut spec, false);

        stdout.fg(&mut spec, Blue);
        print!("Находимся в точке:");
        stdout.fg(&mut spec, Green);
        print!("   {:.4}, {:.4}", info.new_x, info.new_y);
        stdout.fg(&mut spec, Blue);
        print!("\t\tЗначение функции:");
        stdout.fg(&mut spec, Green);
        print!("   {:.6}", info.f);
        stdout.fg(&mut spec, Blue);
        print!("\t\tВычислений функции:");
        stdout.fg(&mut spec, Green);
        print!("   {}", info.calc_metric);
        println!();

        counter += 1;
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
    println!("Нажмите Enter чтобы продолжить...");
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    println!();
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
                TableCell::new(params.a),
            ]),
            Row::new(vec![
                TableCell::new("2. Начальное значение X1: "),
                TableCell::new(params.x1_0),
            ]),
            Row::new(vec![
                TableCell::new("3. Начальное значение X2: "),
                TableCell::new(params.x2_0),
            ]),
            Row::new(vec![
                TableCell::new("4. Начальная длина шага: "),
                TableCell::new(params.initial_step),
            ]),
            Row::new(vec![
                 TableCell::new("5. Алгоритм минимизации: "),
                 TableCell::new(params.method.desc()),
            ]),
            Row::new(vec![
                 TableCell::new("6. Печатать каждые _ шагов: "),
                 TableCell::new(params.print_every),
            ])
        ]
    ).build();

    let mut spec = ColorSpec::new();
    stdout.fg(&mut spec, Blue);
    stdout.intense(&mut spec, true);

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

fn adjust_size() {

    use std::io::{stdout, Write};
    use crossterm::{execute, Result, terminal::{ScrollUp, SetSize, size}};

    execute!(
        stdout(),
        SetSize(100, 100)
    );


    println!("Пожалуйста, увеличьте ширину терминала...");
    loop {
        let dimensions = term_size::dimensions();
        match dimensions {
            Some((x,y)) => {
                if x > 100 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            },
            None => {
                println!("Установите размер терминала, чтобы было видно все элементы интерфейса...");
                let mut buf = String::new();
                let _ = stdin().read_line(&mut buf);
            }
        }
    }
}

fn main() {
    clear_screen();
    adjust_size();
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
    loop {
        print_params(&mut stdout, &params);

        let mut spec = ColorSpec::new();
        stdout.clear(&mut spec);

        println!("Введите номер параметра, который хотите изменить, или нажмите Enter для запуска алгоритма...");
        println!();

        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().is_empty() {
            let f = F(params.a);
        
            algo = match params.method {
                MethodEnum::ConstStep => Box::new(const_step::ConstStep::new(params.initial_step)),
                MethodEnum::DecStep => Box::new(dec_step::DecStep::new(params.initial_step)),
                MethodEnum::SplitStep => Box::new(split_step::SplitStep::new(params.initial_step)),
                MethodEnum::SteepestDescend => Box::new(steepest_descend::SteepestDescend::new(params.initial_step)),
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
                    stdout.fg(&mut spec, Red);
                    println!("Ошибка ввода!");
                    stdout.clear(&mut spec);
                    continue;
                }
            };
            
            run(&f, &mut algo, &mut stdout, params.x1_0, params.x2_0, steps as u64, params.print_every);
        }
        else if let Err(()) = update_param(buffer.trim().parse::<u32>().map_err(|_| ()), &mut params) {
            stdout.fg(&mut spec, Red);
            println!("Ошибка ввода!");
            stdout.clear(&mut spec);
            continue;
        }

    }



}

fn update_param(num: Result<u32, ()>, params: &mut Params) -> Result<(), ()> {
    let mut buffer = String::new();
    let v = num.map_err(|_| ())?;
    match v {
        1 => {
            println!("Введите новое значение коэффициента");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<f64>().map_err(|_| ())?;
            params.a = v;
        },
        2 => {
            println!("Введите новое начальное значение Х1");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<f64>().map_err(|_| ())?;
            params.x1_0 = v;
        },
        3 => {
            println!("Введите новое начальное значение Х2");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<f64>().map_err(|_| ())?;
            params.x2_0 = v;
        },
        4 => {
            println!("Введите новую начальную длину шага");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<f64>().map_err(|_| ())?;
            params.initial_step = v;
        },
        5 => {
            println!("Выберите алгоритм минимизации:");
            println!("1. {}", MethodEnum::ConstStep.desc());
            println!("2. {}", MethodEnum::DecStep.desc());
            println!("3. {}", MethodEnum::SplitStep.desc());
            println!("4. {}", MethodEnum::SteepestDescend.desc());
            stdin().read_line(&mut buffer).unwrap();
            params.method = match buffer.trim().parse::<u32>().map_err(|_| ())? {
                1 => MethodEnum::ConstStep,
                2 => MethodEnum::DecStep,
                3 => MethodEnum::SplitStep,
                4 => MethodEnum::SteepestDescend,
                _ => return Err(())
                
            };
        },
        6 => {
            println!("Печать каждые _ шагов:");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<u64>().map_err(|_| ())?;
            params.print_every = v;
        },
        _ => {
            return Err(())
        }

    }
    Ok(())

}
