// struct Command {
//     op: u8,
//     arg1: u16,
//     arg2: u16,
//     arg3: u16,
// }

// impl Command {
//     fn execute(&self) -> (bool, usize) {
//         match self.op {

//             op => {
//                 panic!("unknown operation {}", op);
//             }
//         }
//         (false, 0)
//     }
// }

// #[derive(Debug)]
// struct Word<const SIZE: usize> {
//     data: [u8; SIZE],
// }

// impl<const LENGTH: usize> Word<LENGTH> {
//     fn read(&self, raw_memory: &[u8], index: usize) {

//     }
// }

pub fn execute(memory: &mut [u64]) {
    let mut addr_counter = 0;

    while addr_counter < memory.len() {
        let (op, addr1, addr2, addr3) = next_command(memory[addr_counter]);
        let (arg1, arg2, arg3) = (
            memory[addr1 as usize],
            memory[addr2 as usize],
            memory[addr3 as usize],
        );

        match op {
            1 => {
                let v = arg2.checked_add(arg3);
            }
            _ => {
                panic!("");
            }
        }
        addr_counter += 1;
    }
}

fn next_command(word: u64) -> (u16, u16, u16, u16) {
    let two_bytes: u64 = 0xFFFF000000000000;
    (
        (word >> 48) as u16,
        ((word >> 32) & two_bytes) as u16,
        ((word >> 16) & two_bytes) as u16,
        (word & two_bytes) as u16,
    )
}

// pub fn execute(memory: &mut [u8]) {
//     let mut addr_counter = 0;
//     let word_size = 8;

//     while addr_counter < memory.len() {
//         let (op, addr_arg1, addr_arg2, addr_arg3) = read_command(&memory[addr_counter..]);
//         let (addr_arg1, addr_arg2, addr_arg3) = (
//             (addr_arg1 * word_size) as usize,
//             (addr_arg2 * word_size) as usize,
//             (addr_arg3 * word_size) as usize,
//         );
//         let (arg1, arg2, arg3) = (
//             read_word::<u64>(&memory[addr_arg1..]),
//             read_word::<u64>(&memory[addr_arg2..]),
//             read_word::<u64>(&memory[addr_arg3..]),
//         );
//         addr_counter += 8;
//     }
// }

// fn read_command(command: &[u8]) -> (u16, u16, u16, u16) {
//     (
//         read_word(&command[..]),
//         read_word(&command[2..]),
//         read_word(&command[4..]),
//         read_word(&command[6..]),
//     )
// }

// fn read_word<T>(command: &[u8]) -> T
// where
//     T: Default
//         + std::convert::From<u8>
//         + std::ops::BitOrAssign
//         + std::ops::Shl<Output = T>
//         + std::ops::AddAssign
//         + Copy,
// {
//     let mut n: T = T::from(0);
//     let mut shift: T = T::from(0);

//     command[..std::mem::size_of::<T>()]
//         .iter()
//         .rev()
//         .for_each(|&x| {
//             n |= (T::from(x)) << shift;
//             shift += T::from(8);
//         });
//     n
// }
