pub fn execute(memory: &mut [u64]) {
    let mut addr_counter = 0;
    let mut machine = Machine::new();

    while addr_counter < memory.len() {
        let (op, addr1, addr2) = next_command(memory[addr_counter]);
        let command = Command { addr1, addr2 };

        match op {
            0x0 => {
                machine.copy_memory(memory, &command);
            }
            0x1 => {
                machine.addition_float(memory, &command);
            }
            0x2 => {
                machine.substruction_float(memory, &command);
            }
            0x3 => {
                machine.mul_float(memory, &command);
            }
            0x4 => {
                machine.div_float(memory, &command);
            }
            0x6 => {
                machine.read_integer_from_stdin(memory, &command);
            }
            0x9 => {
                addr_counter = machine.uncondition_jump(&command);
                continue;
            }
            0xB => {
                machine.addition_int(memory, &command);
            }
            0xC => {
                machine.substruction_int(memory, &command);
            }
            0xD => {
                machine.mul_int(memory, &command);
            }
            0xF => {
                machine.write_float(memory, &command);
            }
            0x15 => {
                let (jump, new_addr_counter) = machine.condition_jump_more(&command);
                if jump {
                    addr_counter = new_addr_counter;
                    continue;
                }
            }
            0x16 => {
                let (jump, new_addr_counter) = machine.condition_jump_less(&command);
                if jump {
                    addr_counter = new_addr_counter;
                    continue;
                }
            }
            0x17 => {
                let (jump, new_addr_counter) = machine.condition_jump_equal(&command);
                if jump {
                    addr_counter = new_addr_counter;
                    continue;
                }
            }
            0x14 => {
                machine.cast_int_to_float(memory, &command);
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
}

struct Machine {
    flag: u64,
    // panic: u64,
    err: u64,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            flag: 0,
            // panic: 0,
            err: 0,
        }
    }

    fn set_flag(&mut self, v: u64) {
        let v = v as i64;
        self.flag = match v.cmp(&0) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
        };
    }

    fn copy_memory(&self, memory: &mut [u64], command: &Command) {
        memory[command.addr1] = memory[command.addr2];
    }

    fn cast_int_to_float(&self, memory: &mut [u64], command: &Command) {
        let f1 = memory[command.addr2] as f64;
        memory[command.addr1] = f1.to_bits();
    }

    fn condition_jump_more(&self, command: &Command) -> (bool, usize) {
        let jump = self.flag == 2;
        (jump, command.addr1 as usize)
    }

    fn condition_jump_less(&self, command: &Command) -> (bool, usize) {
        let jump = self.flag == 1;
        (jump, command.addr1 as usize)
    }

    fn condition_jump_equal(&self, command: &Command) -> (bool, usize) {
        let jump = self.flag == 0;
        (jump, command.addr1 as usize)
    }

    fn uncondition_jump(&self, command: &Command) -> usize {
        command.addr2 as usize
    }

    fn addition_float(&self, memory: &mut [u64], command: &Command) {
        let f1 = f64::from_bits(memory[command.addr1]);
        let f2 = f64::from_bits(memory[command.addr2]);
        memory[command.addr1] = (f1 + f2).to_bits();
    }

    fn substruction_float(&self, memory: &mut [u64], command: &Command) {
        let f1 = f64::from_bits(memory[command.addr1]);
        let f2 = f64::from_bits(memory[command.addr2]);
        memory[command.addr1] = (f1 - f2).to_bits();
    }

    fn write_float(&self, memory: &mut [u64], command: &Command) {
        let mut arg2 = command.addr2;
        let mut addr1 = command.addr1;

        while arg2 != 0 {
            let f1 = f64::from_bits(memory[addr1]);
            println!("{}", f1);
            arg2 -= 1;
            addr1 += 1;
        }
    }

    fn substruction_int(&mut self, memory: &mut [u64], command: &Command) {
        let i1 = memory[command.addr1] as i64;
        let i2 = memory[command.addr2] as i64;
        memory[command.addr1] = i1.wrapping_sub(i2) as u64;
        self.set_flag(memory[command.addr1]);
    }

    fn mul_int(&mut self, memory: &mut [u64], command: &Command) {
        let i1 = memory[command.addr1] as i64;
        let i2 = memory[command.addr2] as i64;
        memory[command.addr1] = i1.wrapping_mul(i2) as u64;
        self.set_flag(memory[command.addr1]);
    }

    fn addition_int(&mut self, memory: &mut [u64], command: &Command) {
        let u1 = memory[command.addr1] as u64;
        let u2 = memory[command.addr2] as u64;
        memory[command.addr1] = u1.wrapping_add(u2) as u64;
        self.set_flag(memory[command.addr1]);
    }

    fn div_float(&self, memory: &mut [u64], command: &Command) {
        let f1 = f64::from_bits(memory[command.addr1]);
        let f2 = f64::from_bits(memory[command.addr2]);
        memory[command.addr1] = (f1 / f2).to_bits();
    }

    fn mul_float(&self, memory: &mut [u64], command: &Command) {
        let f1 = f64::from_bits(memory[command.addr1]);
        let f2 = f64::from_bits(memory[command.addr2]);
        memory[command.addr1] = (f1 * f2).to_bits();
    }

    fn read_integer_from_stdin(&mut self, memory: &mut [u64], command: &Command) {
        let mut buffer = String::new();
        let mut arg2 = command.addr2;
        let mut addr1 = command.addr1;

        while arg2 != 0 {
            buffer.clear();
            let res = std::io::stdin().read_line(&mut buffer);
            if res.is_err() {
                self.err = 1;
                break;
            }

            let res = read_integer_as_bits(buffer.trim());
            if res.is_err() {
                self.err = 1;
                break;
            }

            let res = res.unwrap();
            memory[addr1] = res;
            self.err = 0;
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

fn next_command(word: u64) -> (usize, usize, usize) {
    let two_bytes: u64 = 0x0000000000FFFFFF;
    (
        (word >> 48) as usize,
        ((word >> 24) & two_bytes) as usize,
        (word & two_bytes) as usize,
    )
}
