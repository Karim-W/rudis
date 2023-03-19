pub mod tests;
#[derive(Debug, PartialEq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Sadd(String, String),
    Smembers(String),
    Sismember(String, String),
    Srem(String, String),
    Ping,
    None,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Command::Get(s.to_string()),
            "SET" => Command::Set(s.to_string(), s.to_string()),
            "DEL" => Command::Del(s.to_string()),
            "SADD" => Command::Sadd(s.to_string(), s.to_string()),
            "SMEMBERS" => Command::Smembers(s.to_string()),
            "SMEM" => Command::Smembers("SMEMBERS".to_string()),
            "SISMEMBER" => Command::Sismember(s.to_string(), s.to_string()),
            "SREM" => Command::Srem(s.to_string(), s.to_string()),
            "PING" => Command::Ping,
            _ => Command::None,
        }
    }
}
