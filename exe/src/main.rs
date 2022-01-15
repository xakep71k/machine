use std::io::BufRead;

fn main() {
    if std::env::args().len() != 2 {
        eprintln!("please specify a file with commands");
        std::process::exit(1);
    }

    let (machine_type, text_memory) = read_memory_from_file(&std::env::args().nth(1).unwrap());
    let mut bin_memory = text_to_bin_memory(&text_memory);

    match &machine_type[..] {
        "03" => {
            um3::execute(&mut bin_memory);
        }
        _ => {
            eprintln!("unknown type '{}'", machine_type);
            std::process::exit(1);
        }
    }
}

fn read_memory_from_file(filename: &str) -> (String, Vec<char>) {
    let file = std::fs::File::open(filename).expect("failed to read file");
    let mut input: Vec<String> = Vec::new();

    for line in std::io::BufReader::new(file).lines() {
        let s = line.unwrap().split(';').next().unwrap().trim().to_string();
        if !s.is_empty() {
            input.push(s);
        }
    }

    let machine_type: String = input.first().unwrap().chars().take(2).collect();
    let input = input
        .join("")
        .chars()
        .skip(2)
        .filter(|ch| !ch.is_whitespace())
        .collect::<Vec<char>>();
    (machine_type, input)
}

fn text_to_bin_memory(text_memory: &[char]) -> Vec<u64> {
    let mut bin_memory = vec![0; 65536];
    let mut imemory = 0;
    text_memory
        .iter()
        .enumerate()
        .step_by(16)
        .for_each(|(i, _)| {
            let mut k = i;
            let i = i + 16;
            let mut command: u64 = 0;
            let shift = 4;
            while k < i {
                command <<= shift;
                let v = u64::from_str_radix(&text_memory[k].to_string(), 16).unwrap();
                command |= v;
                k += 1;
            }
            bin_memory[imemory] = command;
            imemory += 1;
        });
    bin_memory
}
