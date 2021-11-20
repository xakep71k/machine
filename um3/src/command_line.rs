#[derive(Debug)]
pub enum MachineType {
    Um1,
    Um2,
    Um3,
    Ums,
}

#[derive(Debug)]
pub enum Argument {
    Help,
    WordSize(usize),
    MachineCommandList,
    MachineType(MachineType),
    Error { message: String },
}

pub fn inform_about_errors(v: Vec<Argument>) {
    let show_help = v.is_empty() || v.iter().any(|x| matches!(x, Argument::Help));
    if show_help {
        print_help();
    }
}

pub fn parse() -> Vec<Argument> {
    let mut args = std::env::args();
    let _program_name: String = args.next().unwrap();
    let mut result: Vec<Argument> = Vec::new();
    while let Some(arg) = args.next() {
        match &arg[..] {
            "--help" => {
                result.push(Argument::Help);
            }
            "--clist" => {
                result.push(Argument::MachineCommandList);
            }
            "--memory-size" => {
                result.push(parse_word_size(args.next()));
            }
            "--machine-type" => {
                result.push(parse_machine_type(args.next()));
            }
            arg => {
                result.push(Argument::Error {
                    message: format!("unknown option '{}'", arg),
                });
            }
        }
    }
    result
}

pub fn print_help() {
    println!("Эмулятор учебных машин УМ-1, УМ-2, УМ-3, УМ-С.");
    println!("УМ-3 - трёхадресная учебная машина");
    println!("УМ-2 - двухадресная учебная машина");
    println!("УМ-1 - одноадресная учебная машина");
    println!("УМ-С - безадресная или стековая учебная машина");
    println!("Опции:");
    println!("\t--help\tСправка");
    println!("\t--clist\tСписок команд процессора");
    println!("\t--machine-type\tВозможные значения: UM3|УМ3|УМ-3 или UM2|УМ2|УМ-2 или UM1|УМ1|УМ-1 или UMS|УМС|УМ-С");
    println!("\t--word-size\tРазмер машинного слова. Например, трёхадресная команда, где КОП - Код ОПерации во всех машинах равен 1 байт (8 разрядов), размеры A1, A2 и A3 равны и зависят от размера машнного слова. |<КОП>|<A1>|<A2>|<A3>| Если машинное слово равно 32 разряда, то с учётом, что КОП = 2, А1 = A2 = A3 =  (32 - 2) / 3 = 10 разрядов. Тогда в этом случае максимальный адрес равен 2^10 = 1024 разряда. См. книгу ");
}

fn parse_word_size(arg: Option<String>) -> Argument {
    if let Some(size) = arg {
        if let Ok(size) = size.parse::<usize>() {
            Argument::WordSize(size)
        } else {
            let message = if is_number(&size) {
                format!("--word-size has too long number '{}'", size)
            } else {
                format!(
                    "--word-size parameter must be a positive number but found '{}'",
                    size
                )
            };
            Argument::Error { message }
        }
    } else {
        Argument::Error {
            message: "--word-size: missing parameter size".to_string(),
        }
    }
}

fn is_number(arg: &str) -> bool {
    arg.chars().all(|ch| ch.is_digit(10))
}

fn parse_machine_type(arg: Option<String>) -> Argument {
    if let Some(mtype) = arg {
        match &mtype.to_lowercase()[..] {
            "ums" | "ум-с" | "умс" => Argument::MachineType(MachineType::Ums),
            "um3" | "ум-3" | "ум3" => Argument::MachineType(MachineType::Um3),
            "um2" | "ум-2" | "ум2" => Argument::MachineType(MachineType::Um2),
            "um1" | "ум-1" | "ум1" => Argument::MachineType(MachineType::Um1),
            mtype => Argument::Error {
                message: format!("unknown machine type '{}', see --help", mtype),
            },
        }
    } else {
        Argument::Error {
            message: "machine type not specified".to_string(),
        }
    }
}
