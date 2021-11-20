#[derive(Debug)]
pub enum MachineType {
    Um1,
    Um2,
    Um3,
    Ums,
}

#[derive(Debug)]
pub enum NumeralSystem {
    Bin,
    Dec,
    Hex,
}

#[derive(Debug)]
pub enum Argument {
    Help,
    AddressSize(usize),
    NumeralSystem(NumeralSystem),
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
            "--numeral" => {
                result.push(parse_numeral_system(args.next()));
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
    println!("Эмулятор учебных машин УМ-1, УМ-2, УМ-3, УМ-С.");
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
    println!("\t--machine-type\tВозможные значения: UM3|УМ3|УМ-3 или UM2|УМ2|УМ-2 или UM1|УМ1|УМ-1 или UMS|УМС|УМ-С");
    println!("\t--address-size\tРазмер секции адреса в команде. От данного параметра зависит размер всей команды. Для УМ-3 длина команды это |<КОП>|<A1>|<A2>|<A3>|, для УМ-2 это |<КОП>|<A1>|<A2>|, для УМ-1 это |<КОП>|<A1>|. А1, А2 и А3 это адреса откуда команда получает значения. Для стековой машины (УМ-С) этот параметр игнорируется. Например, --address-size 10, то команда для УМ-3 равна 32, а максимально адресуемая ячейка памяти 1024. Так как A1 = A2 = A3 = 10, то размер команды будет 32 = |КОП=2|А1=10|А2=10|А3=10|, размер КОП всегда равен 2. См. книгу Баула В.Г. - Введение в архитектуру ЭВМ (2003).pdf, страница 10, https://github.com/xakep71k/machines/blob/master/docs/%D0%91%D0%B0%D1%83%D0%BB%D0%B0%20%D0%92.%D0%93.%20-%20%D0%92%D0%B2%D0%B5%D0%B4%D0%B5%D0%BD%D0%B8%D0%B5%20%D0%B2%20%D0%B0%D1%80%D1%85%D0%B8%D1%82%D0%B5%D0%BA%D1%82%D1%83%D1%80%D1%83%20%D0%AD%D0%92%D0%9C%20(2003).pdf")
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

fn parse_numeral_system(arg: Option<String>) -> Argument {
    let wrong_numeral = Argument::Error {
        message: "wrong numeral system, it must be 2, 10 or 16".to_string(),
    };
    if let Some(arg) = arg {
        if let Ok(n) = arg.parse::<u32>() {
            match n {
                2 => Argument::NumeralSystem(NumeralSystem::Bin),
                10 => Argument::NumeralSystem(NumeralSystem::Dec),
                16 => Argument::NumeralSystem(NumeralSystem::Hex),
                _ => wrong_numeral,
            }
        } else {
            wrong_numeral
        }
    } else {
        wrong_numeral
    }
}
