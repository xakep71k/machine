#[derive(Debug, Clone)]
pub enum Kind {
    Education,
    Binary,
}

pub fn parse(arg: Option<String>) -> Result<Kind, String> {
    if let Some(arg) = arg {
        match &arg[..] {
            "education" => Ok(Kind::Education),
            "binary" => Ok(Kind::Binary),
            arg => Err(format!("unknown format type '{}'", arg)),
        }
    } else {
        Err("format type not specified".to_string())
    }
}
