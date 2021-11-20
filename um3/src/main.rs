mod command_line;
// use num::bigint::BigInt;

fn main() {
    // let args = std::env::args().collect::<Vec<String>>();
    // match "1".parse::<BigInt>() {
    //     Ok(n) => println!("{}", n),
    //     Err(_) => println!("Error"),
    // }
    let opts = command_line::parse();
    println!("=== {:?}", opts);
    command_line::inform_about_errors(opts);
}
