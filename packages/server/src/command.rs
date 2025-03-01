use crate::resp::RespType;

pub enum Command {
    Ping,
}

impl Command {
    pub fn from_resp_type(resp_type: RespType) -> Option<Command> {
        match resp_type {
            RespType::SimpleString(val) if val.as_str() == "PING" => {
                Some(Command::Ping)
            },
            _ => None
        }
    }

    pub fn handle(&self) -> String {
        match self {
            Self::Ping => "PONG".to_string()
        }
    }
}
