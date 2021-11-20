use cmd;

fn main() {
    // let args = std::env::args().collect::<Vec<String>>();
    // match "1".parse::<BigInt>() {
    //     Ok(n) => println!("{}", n),
    //     Err(_) => println!("Error"),
    // }
    let opts = cmd::parse();
    cmd::check_for_errors(opts);
}
