pub struct Data<'a> {
    pub memory: &'a mut [u64],
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub addr1: usize,
    pub addr2: usize,
    pub addr3: usize,
}

pub fn read_integer(data: Data) {
    let mut buffer = String::new();
    let mut arg2 = data.addr2;
    let mut addr1 = data.addr1;

    while arg2 != 0 {
        buffer.clear();
        let res = std::io::stdin().read_line(&mut buffer);
        if res.is_err() {
            data.memory[data.addr3] = 1;
            break;
        }

        let res = read_integer_as_bits(buffer.trim());
        if res.is_err() {
            data.memory[data.addr3] = 1;
            break;
        }

        let res = res.unwrap();
        data.memory[addr1] = res;
        data.memory[data.addr3] = 0;
        arg2 -= 1;
        addr1 += 1;
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
