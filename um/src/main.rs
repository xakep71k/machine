mod help;
mod machine;
mod memory;

fn main() {
    let args = options::parse();
    if args
        .iter()
        .any(|x| matches!(x, options::Argument::Error { .. }))
    {
        args.iter().for_each(|x| {
            if let options::Argument::Error { message } = x {
                eprintln!("{}", message)
            }
        });
        std::process::exit(1);
    }
    if args.is_empty() || args.iter().any(|x| matches!(x, options::Argument::Help)) {
        help::print();
        std::process::exit(0);
    }
    let opts = machine::Opts::from_args(args);
    let mut memory: Vec<u64> = memory::load_from_stdin(opts.format_kind());
    let execute = machine::create(opts.machine_kind());
    execute(&mut memory);
}
