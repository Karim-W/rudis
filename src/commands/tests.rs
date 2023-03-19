#[cfg(test)]
mod tests {
    use crate::commands::Command;
    #[test]
    fn get_works() {
        let cmd = Command::from("GET");
        assert_eq!(cmd, Command::Get("GET".to_string()));
    }

    #[test]
    fn set_works() {
        let cmd = Command::from("SET");
        assert_eq!(cmd, Command::Set("SET".to_string(), "SET".to_string()));
    }

    #[test]
    fn sadd_works() {
        let cmd = Command::from("SADD");
        assert_eq!(cmd, Command::Sadd("SADD".to_string(), "SADD".to_string()));
    }

    #[test]
    fn smembers_works() {
        let cmd = Command::from("SMEMBERS");
        assert_eq!(cmd, Command::Smembers("SMEMBERS".to_string()));
    }

    #[test]
    fn smembers_2_works() {
        let cmd = Command::from("SMEM");
        assert_eq!(cmd, Command::Smembers("SMEMBERS".to_string()));
    }

    #[test]
    fn sismember_works() {
        let cmd = Command::from("SISMEMBER");
        assert_eq!(
            cmd,
            Command::Sismember("SISMEMBER".to_string(), "SISMEMBER".to_string())
        );
    }

    #[test]
    fn srem_works() {
        let cmd = Command::from("SREM");
        assert_eq!(cmd, Command::Srem("SREM".to_string(), "SREM".to_string()));
    }

    #[test]
    fn none_works() {
        let cmd = Command::from("NONE");
        assert_eq!(cmd, Command::None);
    }

    #[test]
    fn none_works_2() {
        let cmd = Command::from("Foo");
        assert_ne!(cmd, Command::Get("GET".to_string()));
    }

    #[test]
    fn ping_works() {
        let cmd = Command::from("PING");
        assert_eq!(cmd, Command::Ping);
    }

    #[test]
    fn ping_works_2() {
        let cmd = Command::from("PING");
        assert_ne!(cmd, Command::Get("GET".to_string()));
    }

    #[test]
    fn del_works() {
        let cmd = Command::from("DEL");
        assert_eq!(cmd, Command::Del("DEL".to_string()));
    }

    #[test]
    fn del_works_2() {
        let cmd = Command::from("DEL");
        assert_ne!(cmd, Command::Get("GET".to_string()));
    }
}
