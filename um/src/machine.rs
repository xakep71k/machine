pub fn create(kind: &arguments::machine::Kind) -> Box<dyn Fn(&mut Vec<u64>)> {
    match kind {
        arguments::machine::Kind::UMS => Box::new(|x| {
            ums::execute(x);
        }),
        arguments::machine::Kind::UM1 => Box::new(|x| {
            um1::execute(x);
        }),
        arguments::machine::Kind::UM2 => Box::new(|x| {
            um2::execute(x);
        }),
        arguments::machine::Kind::UM3 => Box::new(|x| {
            um3::execute(x);
        }),
    }
}

pub struct Opts {
    fkind: arguments::format::Kind,
    mkind: arguments::machine::Kind,
}

impl Opts {
    pub fn from_args(args: Vec<arguments::Kind>) -> Opts {
        let fkind = args.iter().find_map(|x| {
            if let arguments::Kind::Format(kind) = x {
                Some(kind)
            } else {
                None
            }
        });
        let mkind = args.iter().find_map(|x| {
            if let arguments::Kind::Machine(kind) = x {
                Some(kind)
            } else {
                None
            }
        });
        Opts {
            fkind: fkind.cloned().unwrap_or(arguments::format::Kind::Binary),
            mkind: mkind.cloned().unwrap_or(arguments::machine::Kind::UMS),
        }
    }

    pub fn format_kind(&self) -> &arguments::format::Kind {
        &self.fkind
    }

    pub fn machine_kind(&self) -> &arguments::machine::Kind {
        &self.mkind
    }
}
