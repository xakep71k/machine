pub mod format;
pub mod machine;

pub struct Machine {
    fkind: format::Kind,
    mkind: machine::Kind,
}

#[derive(Debug)]
pub enum Argument {
    Help,
    Format(format::Kind),
    MachineCommandList,
    Machine(machine::Kind),
    Error { message: String },
}

impl Machine {
    pub fn from_args(args: Vec<Argument>) -> Machine {
        let fkind = args.iter().find_map(|x| {
            if let Argument::Format(kind) = x {
                Some(kind)
            } else {
                None
            }
        });
        let mkind = args.iter().find_map(|x| {
            if let Argument::Machine(kind) = x {
                Some(kind)
            } else {
                None
            }
        });
        Machine {
            fkind: fkind.cloned().unwrap_or(format::Kind::Binary),
            mkind: mkind.cloned().unwrap_or(machine::Kind::UMS),
        }
    }

    pub fn format_kind(&self) -> &format::Kind {
        &self.fkind
    }

    pub fn machine_kind(&self) -> &machine::Kind {
        &self.mkind
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
            "--format" => {
                let arg = match format::parse(args.next()) {
                    Ok(kind) => Argument::Format(kind),
                    Err(message) => Argument::Error { message },
                };
                result.push(arg);
            }
            "--machine-type" => {
                let arg = match machine::parse_type(args.next()) {
                    Ok(kind) => Argument::Machine(kind),
                    Err(message) => Argument::Error { message },
                };
                result.push(arg);
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
