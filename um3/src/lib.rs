struct Machine {}

trait Operation {
    fn execute() -> (bool, usize);
}

struct OperationUM3 {
    op: u8,
    arg1: u16,
    arg2: u16,
    arg3: u16,
}

impl Operation for OperationUM3 {
    fn execute() -> (bool, usize) {
        (false, 0)
    }
}

pub fn execute(memory: &mut [u8]) {
    let mut commands = split_commands(memory);
    let mut i = 0;

    while i < commands.len() {
        let cmd: &[u8] = &commands[i];
        let cmd = cmd.try_into().expect("never happened");
        let (op, arg1, arg2, arg3) = parse_command(cmd);
        i += 1;
    }
}

fn parse_command(command: &[u8; 7]) -> (u8, u16, u16, u16) {
    (
        command[0],
        (command[1] << 1 | command[2]).into(),
        (command[3] << 1 | command[4]).into(),
        (command[5] << 1 | command[6]).into(),
    )
}

fn split_commands(memory: &mut [u8]) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < memory.len() {
        let command_len = 7;
        let mut j = 0;
        let mut command = Vec::new();

        while j < command_len {
            command.push(memory[i + j]);
            j += 1;
        }

        result.push(command);
        i += command_len;
    }

    result
}
