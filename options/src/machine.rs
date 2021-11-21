#[derive(Debug, Clone)]
pub enum Kind {
    UMS,
    UM1,
    UM2,
    UM3,
}

pub fn parse_type(arg: Option<String>) -> Result<Kind, String> {
    if let Some(mtype) = arg {
        match &mtype.to_lowercase()[..] {
            "ums" | "ум-с" | "умс" => Ok(Kind::UMS),
            "um3" | "ум-3" | "ум3" => Ok(Kind::UM3),
            "um2" | "ум-2" | "ум2" => Ok(Kind::UM2),
            "um1" | "ум-1" | "ум1" => Ok(Kind::UM1),
            mtype => Err(format!("unknown machine type '{}', see --help", mtype)),
        }
    } else {
        Err("machine type not specified".to_string())
    }
}
