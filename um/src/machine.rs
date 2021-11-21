pub fn create(kind: &options::machine::Kind) -> Box<dyn Fn(&mut Vec<u64>)> {
    match kind {
        options::machine::Kind::UMS => Box::new(|x| {
            ums::execute(x);
        }),
        options::machine::Kind::UM1 => Box::new(|x| {
            um1::execute(x);
        }),
        options::machine::Kind::UM2 => Box::new(|x| {
            um2::execute(x);
        }),
        options::machine::Kind::UM3 => Box::new(|x| {
            um3::execute(x);
        }),
    }
}

pub struct Opts {
    fkind: options::format::Kind,
    mkind: options::machine::Kind,
}

impl Opts {
    pub fn from_args(args: Vec<options::Argument>) -> Opts {
        let fkind = args.iter().find_map(|x| {
            if let options::Argument::Format(kind) = x {
                Some(kind)
            } else {
                None
            }
        });
        let mkind = args.iter().find_map(|x| {
            if let options::Argument::Machine(kind) = x {
                Some(kind)
            } else {
                None
            }
        });
        Opts {
            fkind: fkind.cloned().unwrap_or(options::format::Kind::Binary),
            mkind: mkind.cloned().unwrap_or(options::machine::Kind::UMS),
        }
    }

    pub fn format_kind(&self) -> &options::format::Kind {
        &self.fkind
    }

    pub fn machine_kind(&self) -> &options::machine::Kind {
        &self.mkind
    }
}
