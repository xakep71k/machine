pub fn execute(memory: &mut [u64]) {
    let mut addr_counter = 0;
    let mut machine = Machine::new();

    while addr_counter < memory.len() {
        let (op, addr1) = next_command(memory[addr_counter]);
        let command = Command { addr1 };

        match op {
            0x0 => {
                machine.push_from_memory(memory, &command);
            }
            0x18 => {
                machine.pop_to_memory(memory, &command);
            }
            0x1 => {
                machine.addition_float();
            }
            0x2 => {
                machine.substruction_float();
            }
            0x3 => {
                machine.mul_float();
            }
            0x4 => {
                machine.div_float();
            }
            0x6 => {
                machine.read_integer_from_stdin();
            }
            0x9 => {
                addr_counter = machine.uncondition_jump(&command);
                continue;
            }
            0xB => {
                machine.addition_int();
            }
            0xC => {
                machine.substruction_int();
            }
            0xD => {
                machine.mul_int();
            }
            0xF => {
                machine.write_float();
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
            0x19 => {
                machine.dup_top_stack();
            }
            0x14 => {
                machine.cast_int_to_float();
            }
            0x1A => {
                machine.pop_top();
            }
            0x1F => {
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
}

struct Machine {
    flag: u64,
    // panic: u64,
    err: u64,
    stack: Vec<u64>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            flag: 0,
            // panic: 0,
            err: 0,
            stack: Vec::new(),
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

    fn push_from_memory(&mut self, memory: &mut [u64], command: &Command) {
        self.stack.push(memory[command.addr1]);
    }

    fn pop_to_memory(&mut self, memory: &mut [u64], command: &Command) {
        let value = self.stack.pop().unwrap();
        memory[command.addr1] = value;
    }

    fn cast_int_to_float(&mut self) {
        let value = self.stack.pop().unwrap();
        self.stack.push((value as f64).to_bits());
    }

    fn dup_top_stack(&mut self) {
        let value = *self.stack.last().unwrap();
        self.stack.push(value);
    }

    fn pop_top(&mut self) {
        self.stack.pop().unwrap();
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
        command.addr1 as usize
    }

    fn addition_float(&mut self) {
        let f1 = f64::from_bits(self.stack.pop().unwrap());
        let f2 = f64::from_bits(self.stack.pop().unwrap());
        self.stack.push((f1 + f2).to_bits());
    }

    fn substruction_float(&mut self) {
        let f1 = f64::from_bits(self.stack.pop().unwrap());
        let f2 = f64::from_bits(self.stack.pop().unwrap());
        self.stack.push((f1 - f2).to_bits());
    }

    fn write_float(&mut self) {
        let mut count = self.stack.pop().unwrap();

        while count != 0 {
            println!("{}", f64::from_bits(self.stack.pop().unwrap()));
            count -= 1;
        }
    }

    fn substruction_int(&mut self) {
        let u1 = self.stack.pop().unwrap();
        let u2 = self.stack.pop().unwrap();
        let r = u1.wrapping_sub(u2);
        self.stack.push(r);
        self.set_flag(r);
    }

    fn mul_int(&mut self) {
        let u1 = self.stack.pop().unwrap();
        let u2 = self.stack.pop().unwrap();
        let r = u1.wrapping_mul(u2);
        self.stack.push(r);
        self.set_flag(r);
    }

    fn addition_int(&mut self) {
        let u1 = self.stack.pop().unwrap();
        let u2 = self.stack.pop().unwrap();
        let r = u1.wrapping_add(u2);
        self.stack.push(r);
        self.set_flag(r);
    }

    fn div_float(&mut self) {
        let f1 = f64::from_bits(self.stack.pop().unwrap());
        let f2 = f64::from_bits(self.stack.pop().unwrap());
        self.stack.push((f1 / f2).to_bits());
    }

    fn mul_float(&mut self) {
        let f1 = f64::from_bits(self.stack.pop().unwrap());
        let f2 = f64::from_bits(self.stack.pop().unwrap());
        self.stack.push((f1 * f2).to_bits());
    }

    fn read_integer_from_stdin(&mut self) {
        let mut buffer = String::new();
        let mut count = self.stack.pop().unwrap();

        while count > 0 {
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
            self.stack.push(res);
            self.err = 0;
            count -= 1;
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

fn next_command(word: u64) -> (usize, usize) {
    let mask: u64 = 0x0000FFFFFFFFFFFF;
    ((word >> 48) as usize, (word & mask) as usize)
}
