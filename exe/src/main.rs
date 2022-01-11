use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut memory: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        memory.push(line.unwrap().split(';').next().unwrap().trim().to_string());
    }

    memory.iter().for_each(|x| {
        println!("{}", x);
    });

    let mtype: String = memory[0].chars().take(2).collect();
    match &mtype[..] {
        "03" => {
            println!("УМ-3");
        }
        _ => {
            eprintln!("unknown type {}", mtype);
            std::process::exit(1);
        }
    }
}
