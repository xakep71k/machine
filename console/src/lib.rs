#[derive(Debug)]
pub enum MachineType {
    Um1,
    Um2,
    Um3,
    Ums,
}

#[derive(Debug)]
pub enum Format {
    Education,
    Binary,
}

#[derive(Debug)]
pub enum Argument {
    Help,
    AddressSize(usize),
    Format(Format),
    MachineCommandList,
    MachineType(MachineType),
    Error { message: String },
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
            "--address-size" => {
                result.push(parse_address_size(args.next()));
            }
            "--format" => {
                result.push(parse_format(args.next()));
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
    check_for_errors(&result);
    result
}

fn check_for_errors(v: &Vec<Argument>) {
    v.iter().for_each(|x| {
        if let Argument::Error { message } = x {
            eprintln!("{}", message);
        }
    });
    let errors_exist = v.iter().any(|x| matches!(x, Argument::Error { .. }));
    if errors_exist {
        eprintln!("see --help");
        std::process::exit(1);
    }
    let show_help = v.is_empty() || v.iter().any(|x| matches!(x, Argument::Help));
    if show_help {
        if v.is_empty() {
            eprintln!("no option specified");
        }
        print_help();
        std::process::exit(0);
    }
}

pub fn print_help() {
    println!(
        "Эмулятор учебных машин УМ-1, УМ-2, УМ-3, УМ-С. См. https://github.com/xakep71k/machine"
    );
    println!("УМ-3 - трёхадресная учебная машина");
    println!("УМ-2 - двухадресная учебная машина");
    println!("УМ-1 - одноадресная учебная машина");
    println!("УМ-С - безадресная или стековая учебная машина");
    println!("Опции:");
    println!("\t--help\tСправка");
    println!("\t--clist\tСписок команд процессора");
    println!(
        "\t--numeral\tСистема счисления, в которой задаются команды: 2|10|16, по-умолчанию равно 2"
    );
    println!("\t--format\tВозмжные значения: education|binary. Education - это формат где использется форматирование и команды записаны в десятичной системе счисления, формат текстовый. См. пример ниже. Binary - бинарный формат, состоящий из машиных слов длинной 64 бита, тут используется двоичный бинарный формат. Если формат не указывать, то по умолчанию будет использоваться binary.");
    println!("\t--machine-type\tВозможные значения: UM3|УМ3|УМ-3 или UM2|УМ2|УМ-2 или UM1|УМ1|УМ-1 или UMS|УМС|УМ-С");
}

fn parse_address_size(arg: Option<String>) -> Argument {
    if let Some(size) = arg {
        if let Ok(size) = size.parse::<usize>() {
            Argument::AddressSize(size)
        } else {
            let message = if is_number(&size) {
                format!("--address-size has too long number '{}'", size)
            } else {
                format!(
                    "--address-size parameter must be a positive number but found '{}'",
                    size
                )
            };
            Argument::Error { message }
        }
    } else {
        Argument::Error {
            message: "--address-size: missing parameter size".to_string(),
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

fn parse_format(arg: Option<String>) -> Argument {
    if let Some(arg) = arg {
        match &arg[..] {
            "education" => Argument::Format(Format::Education),
            "binary" => Argument::Format(Format::Binary),
            x => Argument::Error {
                message: format!("unknown format type '{}'", x),
            },
        }
    } else {
        Argument::Error {
            message: "format type not specified".to_string(),
        }
    }
}
