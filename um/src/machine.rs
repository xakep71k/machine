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
