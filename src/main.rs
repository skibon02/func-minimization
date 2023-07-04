
//function: (x2-x1^2)^2+a*(x1-1)^2
mod minimize_methods;

use crossterm::style::{SetForegroundColor, SetBackgroundColor, Print, ResetColor, Color};
use minimize_methods::{MinimizableFunc, MinimizeWorker};
use minimize_methods::*;
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};

use std::cmp::max;
use std::io::stdin;

use term_table::{self, TableBuilder, TableStyle};

use crossterm::style::Stylize;

use std::io::stdout;
use crossterm::{execute, terminal::SetSize};

use crate::minimize_methods::MinimizeMethod;

#[derive(Debug)]
struct F (f64);

fn is_legacy() -> bool {
    match nt_version::get() {
        (6, _, _) => true,
        _ => false
    }
}

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


fn run(f: &impl MinimizableFunc, algo: &mut Box<dyn MinimizeMethod>, start_x: f64, start_y: f64, steps: u64, print_every: u64, algo_desc: &str) -> (f64, f64) {
    let mut res = (start_x, start_y);

    let mut worker = MinimizeWorker::new(f, algo.as_mut()).with_cnt(steps as usize).with_start_point(start_x, start_y);
    let mut total_f_calls = 0;
    let mut total_deriv_calls = 0;

    let mut counter = 0;
    println!();
    println!("Запускаем метод {} с начальной точкой ({:.2}, {:.2})\n", algo_desc, start_x, start_y);
    print!("{}", "0.".blue());
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        );
    println!("\t\t{:.4}, {:.4}\t\t{}{:.6}", start_x, start_y, "f = ".dark_magenta(), f.calc(start_x, start_y).to_string().magenta());
    let mut local_f_calls = 0;
    while let Some(info) = worker.run_step() {
        total_f_calls += info.calc_metric;
        total_deriv_calls += info.deriv_metric;
        counter += 1;
        local_f_calls += info.calc_metric;

        res = (info.new_x, info.new_y);

        if counter % print_every != 0 {
            continue;
        }

        if is_legacy() {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(format!("{}. ", counter)),
                ResetColor
                );
        }
        else {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Blue),
                Print(format!("{}. ", counter)),
                ResetColor
                );
        }

        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Magenta),
            Print(format!("\t\t{:.4}, {:.4}", info.new_x, info.new_y)),
            ResetColor
            );

        if is_legacy() {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Magenta),
            );
            print!("  \tf = ");
        }
        else {
            print!("{}","  \tf = ".dark_magenta());
        }


        if is_legacy() {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Magenta),
                Print(format!("{:.6}", info.f)),
                SetForegroundColor(Color::Yellow),
                );
    
            print!("\t\tВычислений функции:     ");
            print!("{}", local_f_calls);
        }
        else {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Magenta),
                Print(format!("{:.6}", info.f)),
                SetForegroundColor(Color::DarkYellow),
                );
    
            print!("\t\tВычислений функции:     ");
            print!("{}", local_f_calls.to_string().yellow());
        }
        println!();

        local_f_calls = 0;
    }
    println!();

    if is_legacy() {
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            );
    }
    else {
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Blue),
            );
    }
    print!("Функция f была вычислена \t\t");
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        );
    print!("{} раз", total_f_calls);
    
    println!();

    if is_legacy() {
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            );
    }
    else {
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Blue),
            );
    }
    print!("Производная функции f была вычислена \t");
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        );
    print!("{} раз", total_deriv_calls);

    let _ = execute!(
        stdout(),
        ResetColor
        );
    println!();
    println!("Нажмите Enter чтобы продолжить...");
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    println!();
    println!();

    res
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
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Black),
        SetBackgroundColor(Color::Black),
        ResetColor,
        );
    let _ = execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
    let _ = execute!(stdout(), crossterm::cursor::MoveTo(0,0));
}
fn print_params(params: &Params) {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        );
    let params = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new("1. Коеффициент А:"),
                TableCell::new(params.a),
            ]),
            Row::new(vec![
                TableCell::new("2. Начальное значение X1, X2: "),
                TableCell::new(format!("{:.2}, {:.2}", params.x1_0, params.x2_0)),
            ]),
            Row::new(vec![
                TableCell::new("3. Начальная длина шага: "),
                TableCell::new(params.initial_step),
            ]),
            Row::new(vec![
                 TableCell::new("4. Алгоритм минимизации: "),
                 TableCell::new(params.method.desc()),
            ]),
            Row::new(vec![
                 TableCell::new("5. Печатать каждые _ шагов: "),
                 TableCell::new(params.print_every),
            ])
        ]
    ).build();

    println!("{}", params.render());
    let _ = execute!(
        stdout(),
        ResetColor
        );
}

fn print_actions(last_end_point: &Option<(f64, f64)>) {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        );

    let rows = match last_end_point {
        Some((x, y)) => vec![
            Row::new(vec![
                TableCell::new("7. Установить точку из прошлого запуска"),
                TableCell::new(format!("{:.2}, {:.2}", x, y)),
            ]),
            Row::new(vec![
                TableCell::new("8. Выйти из программы"),
            ]),
        ],
        None => vec![
            Row::new(vec![
                TableCell::new("8. Выйти из программы"),
            ]),
        ]
    };
    let params = TableBuilder::new().style(TableStyle::extended()).rows(rows).build();

    println!("{}", params.render());
    let _ = execute!(
        stdout(),
        ResetColor
        );
}

fn print_title() {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        );

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

    println!("{}", table.render());
    let _ = execute!(
        stdout(),
        ResetColor
        );
}

const MIN_TERM_X: u16 = 94;
const MIN_TERM_Y: u16 = 26;

fn adjust_size() {


    // adjust only if we sure that terminal width is small
    let dimensions = term_size::dimensions();
    if let Some(dimensions) = dimensions {
       if dimensions.0 < MIN_TERM_X as usize || dimensions.1 < MIN_TERM_Y as usize{
           let res = execute!(
               stdout(),
               SetSize(max(MIN_TERM_X, dimensions.0 as u16), max(MIN_TERM_Y, dimensions.1 as u16))
           );

           if res.is_ok() {
               return;
           }
       } 
       else {
           return;
       }
    }


    println!("Пожалуйста, увеличьте ширину терминала...");
    loop {
        let dimensions = term_size::dimensions();
        match dimensions {
            Some((x,y)) => {
                if x >= MIN_TERM_X as usize {
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
    
    let mut algo: Box<dyn MinimizeMethod>;
    let mut last_end_point: Option<(f64, f64)> = None;

    let mut params = Params {
        a: 0.1,
        x1_0: 0.0,
        x2_0: 0.0,
        method: MethodEnum::ConstStep,
        print_every: 1,
        initial_step: 0.01,
    };

    let mut buffer = String::new();

    print_title();
    loop {
        print_params(&params);
        print_actions(&last_end_point);

        // stdout.clear();

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
                    execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
                    println!("Ошибка ввода!");
                    execute!(stdout(), ResetColor).unwrap();
                    continue;
                }
            };
            
            last_end_point = Some(
                run(&f, &mut algo, params.x1_0, params.x2_0, steps as u64, params.print_every, params.method.desc()));
        }
        else if let Err(()) = update_param(buffer.trim().parse::<u32>().map_err(|_| ()), &mut params, &mut last_end_point) {
            execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
            println!("Ошибка ввода!");
            execute!(stdout(), ResetColor).unwrap();
            continue;
        }

    }



}

fn update_param(num: Result<u32, ()>, params: &mut Params, last_end_point: &mut Option<(f64, f64)>) -> Result<(), ()> {
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
            buffer.clear();

            println!("Введите новое начальное значение Х2");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<f64>().map_err(|_| ())?;
            params.x2_0 = v;
        },
        3 => {
            println!("Введите новую начальную длину шага");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<f64>().map_err(|_| ())?;
            params.initial_step = v;
        },
        4 => {
            execute!(stdout(), SetForegroundColor(Color::Blue)).unwrap();
            println!("\nВыберите алгоритм минимизации:");
            execute!(stdout(), SetForegroundColor(Color::Cyan)).unwrap();
            println!("1. {}", MethodEnum::ConstStep.desc());
            println!("2. {}", MethodEnum::DecStep.desc());
            println!("3. {}", MethodEnum::SplitStep.desc());
            println!("4. {}", MethodEnum::SteepestDescend.desc());

            execute!(stdout(), ResetColor).unwrap();
            stdin().read_line(&mut buffer).unwrap();
            params.method = match buffer.trim().parse::<u32>().map_err(|_| ())? {
                1 => MethodEnum::ConstStep,
                2 => MethodEnum::DecStep,
                3 => MethodEnum::SplitStep,
                4 => MethodEnum::SteepestDescend,
                _ => return Err(())
                
            };
        },
        5 => {
            println!("Печать каждые _ шагов:");
            stdin().read_line(&mut buffer).unwrap();
            let v = buffer.trim().parse::<u64>().map_err(|_| ())?;
            params.print_every = v;
        },

        7 => {
            if let Some((x1, x2)) = last_end_point {
                params.x1_0 = *x1;
                params.x2_0 = *x2;
                *last_end_point = None;
            }
            else {
                execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
                println!("Нет предыдущей точки!");
                execute!(stdout(), ResetColor).unwrap();
            }
        }

        8 => {
            let _ = execute!(stdout(),
                ResetColor,
                );
            std::process::exit(0);
        }
        _ => {
            return Err(())
        }

    }
    Ok(())

}
