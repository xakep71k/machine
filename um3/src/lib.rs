pub fn execute(memory: &mut [u64]) {
    let mut addr_counter = 0;
    let machine = Machine::new();

    while addr_counter < memory.len() {
        let (op, addr1, addr2, addr3) = next_command(memory[addr_counter]);
        let command = Command {
            addr1,
            addr2,
            addr3,
        };

        match op {
            0x2 => {
                machine.substruction_float(memory, &command);
            }
            0x6 => {
                machine.read_integer_from_stdin(memory, &command);
            }
            0x01F => {
                break;
            }
            _ => {
                panic!("unknown command {}", op);
            }
        }
        addr_counter += 1;
    }
}

struct Command {
    addr1: usize,
    addr2: usize,
    addr3: usize,
}

struct Machine {}

impl Machine {
    fn new() -> Machine {
        Machine {}
    }

    fn substruction_float(&self, memory: &mut [u64], command: &Command) {
        let f1 = f64::from_bits(memory[command.addr2]);
        let f2 = f64::from_bits(memory[command.addr3]);
        memory[command.addr1] = (f1 - f2).to_bits();
    }

    fn read_integer_from_stdin(&self, memory: &mut [u64], command: &Command) {
        let mut buffer = String::new();
        let mut arg2 = command.addr2;
        let mut addr1 = command.addr1;

        while arg2 != 0 {
            buffer.clear();
            let res = std::io::stdin().read_line(&mut buffer);
            if res.is_err() {
                memory[command.addr3] = 1;
                break;
            }

            let res = read_integer_as_bits(buffer.trim());
            if res.is_err() {
                memory[command.addr3] = 1;
                break;
            }

            let res = res.unwrap();
            memory[addr1] = res;
            memory[command.addr3] = 0;
            arg2 -= 1;
            addr1 += 1;
        }
    }
}

fn read_integer_as_bits(input: &str) -> Result<u64, String> {
    if input.starts_with('-') {
        match input.parse::<i64>() {
            Ok(v) => Ok(v as u64),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match input.parse::<u64>() {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_string()),
        }
    }
}

fn next_command(word: u64) -> (usize, usize, usize, usize) {
    let two_bytes: u64 = 0x000000000000FFFF;
    (
        (word >> 48) as usize,
        ((word >> 32) & two_bytes) as usize,
        ((word >> 16) & two_bytes) as usize,
        (word & two_bytes) as usize,
    )
}
