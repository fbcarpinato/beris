use std::fmt::Display;

#[derive(Debug)]
pub enum RespType {
    /*
     * Example: "+hello world\r\n"
     */
    SimpleString(String),
}

impl Display for RespType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimpleString(value) => {
                write!(f, "SimpleString: {}", value)
            }
        }
    }
}

impl RespType {
    pub fn from_vec(bytes: Vec<u8>) -> Result<RespType, String> {
        let line = String::from_utf8_lossy(&bytes);
        println!("Line: {}", line);

        let prefix = line.chars().next().ok_or("Empty input")?;
        let content = &line[1..].trim_end();

        match prefix {
            '+' => {
                Ok(RespType::SimpleString(content.to_string()))
            },
            _ => Err(format!("Unknown RESP prefix {}", prefix))
        }
    }
}
