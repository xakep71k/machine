mod commands;

pub fn execute(memory: &mut [u64]) {
    let mut addr_counter = 0;

    while addr_counter < memory.len() {
        // println!("{:016x}", memory[addr_counter]);
        let (op, addr1, addr2, addr3) = next_command(memory[addr_counter]);
        // println!("op {:016x}", op);
        // println!("addr1 {:016x}", addr1);
        // println!("addr2 {:016x}", addr2);
        // println!("addr3 {:016x}", addr3);
        // let (arg1, arg2, arg3) = (memory[addr1], memory[addr2], memory[addr3]);

        let data = commands::Data {
            memory,
            addr1,
            addr2,
            addr3,
        };

        match op {
            0x6 => {
                commands::read_integer(data);
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

fn next_command(word: u64) -> (usize, usize, usize, usize) {
    let two_bytes: u64 = 0x000000000000FFFF;
    (
        (word >> 48) as usize,
        ((word >> 32) & two_bytes) as usize,
        ((word >> 16) & two_bytes) as usize,
        (word & two_bytes) as usize,
    )
}
