pub mod format;
pub mod machine;

#[derive(Debug)]
pub enum Kind {
    Help,
    Format(format::Kind),
    MachineCommandList,
    Machine(machine::Kind),
    Error { message: String },
}

pub fn parse() -> Vec<Kind> {
    let mut args = std::env::args();
    let _program_name: String = args.next().unwrap();
    let mut result: Vec<Kind> = Vec::new();
    while let Some(arg) = args.next() {
        match &arg[..] {
            "--help" => {
                result.push(Kind::Help);
            }
            "--clist" => {
                result.push(Kind::MachineCommandList);
            }
            "--format" => {
                let arg = match format::parse(args.next()) {
                    Ok(kind) => Kind::Format(kind),
                    Err(message) => Kind::Error { message },
                };
                result.push(arg);
            }
            "--machine-type" => {
                let arg = match machine::parse_type(args.next()) {
                    Ok(kind) => Kind::Machine(kind),
                    Err(message) => Kind::Error { message },
                };
                result.push(arg);
            }
            arg => {
                result.push(Kind::Error {
                    message: format!("unknown option '{}'", arg),
                });
            }
        }
    }
    result
}
