#[cfg(test)]
mod tests {
    use crate::store::Store;

    #[test]
    fn set_and_get() {
        let mut cacher = Store::new();
        cacher.set("foo", "bar");
        assert_eq!(cacher.get("foo"), Some("bar"));
    }

    #[test]
    fn multiple_set_and_get() {
        let mut cacher = Store::new();
        cacher.set("foo", "bar");
        assert_eq!(cacher.get("foo"), Some("bar"));
        cacher.set("foo", "baz");
        assert_eq!(cacher.get("foo"), Some("baz"));
        assert_ne!(cacher.get("foo"), Some("bar"));
        assert_eq!(cacher.get("bar"), None);
    }
    #[test]
    fn not_found_handling() {
        let cacher = Store::new();
        assert_eq!(cacher.get("foo"), None);
        assert_eq!(cacher.get("bar"), None);
        assert_eq!(cacher.get("baz"), None);
        assert_eq!(cacher.get("qux"), None);
    }
    #[test]
    fn sadd() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        assert_eq!(cacher.sismember("foo", "bar"), true);
        assert_eq!(cacher.sismember("foo", "baz"), true);
        assert_eq!(cacher.sismember("foo", "qux"), true);
        assert_eq!(cacher.sismember("foo", "quux"), false);
        assert_eq!(cacher.sismember("bar", "quux"), false);
    }

    #[test]
    fn srem() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        assert_eq!(cacher.sismember("foo", "bar"), true);
        assert_eq!(cacher.sismember("foo", "baz"), true);
        assert_eq!(cacher.sismember("foo", "qux"), true);
        assert_eq!(cacher.sismember("foo", "quux"), false);
        assert_eq!(cacher.sismember("bar", "quux"), false);
        cacher.srem("foo", "bar");
        assert_eq!(cacher.sismember("foo", "bar"), false);
    }

    #[test]
    fn smembers() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        assert_eq!(
            cacher.smembers("foo"),
            Some(&vec![
                "bar".to_string(),
                "baz".to_string(),
                "qux".to_string()
            ])
        );
        assert_eq!(cacher.smembers("bar"), None);
    }

    #[test]
    fn smembers_duplicates() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        assert_eq!(
            cacher.smembers("foo"),
            Some(&vec![
                "bar".to_string(),
                "baz".to_string(),
                "qux".to_string()
            ])
        );
        assert_eq!(cacher.smembers("bar"), None);
    }

    #[test]
    fn smembers_empty() {
        let cacher = Store::new();
        assert_eq!(cacher.smembers("foo"), None);
        assert_eq!(cacher.smembers("bar"), None);
    }

    #[test]
    fn sadd_and_smembers() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        assert_eq!(
            cacher.smembers("foo"),
            Some(&vec![
                "bar".to_string(),
                "baz".to_string(),
                "qux".to_string()
            ])
        );
        assert_eq!(cacher.smembers("bar"), None);
    }

    #[test]
    fn exec_get() {
        let mut cacher = Store::new();
        cacher.set("foo", "bar");
        let res = cacher.exec("GET foo");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), Some("bar".to_string()));
    }

    #[test]
    fn exec_sadd() {
        let mut cacher = Store::new();
        let res = cacher.exec("SADD foo bar");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), None);
        let res = cacher.exec("SMEM foo");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), Some(format!("{:?}", vec!["bar"])));
    }

    #[test]
    fn exec_smem() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        let res = cacher.exec("SMEMBERS foo");
        assert_eq!(res.is_ok(), true);
        let sucess_res = &vec!["bar".to_string(), "baz".to_string(), "qux".to_string()];
        assert_eq!(res.unwrap(), Some(format!("{:?}", sucess_res)));
    }

    #[test]
    fn exec_srem() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        let res = cacher.exec("SREM foo bar");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), None);
        let res = cacher.exec("SMEM foo");
        assert_eq!(res.is_ok(), true);
        let sucess_res = &vec!["baz".to_string(), "qux".to_string()];
        assert_eq!(res.unwrap(), Some(format!("{:?}", sucess_res)));
    }

    #[test]
    fn exec_s_is_members() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        let res = cacher.exec("SISMEMBER foo bar");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), Some("true".to_string()));
        let res = cacher.exec("SISMEMBER foo quux");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), Some("false".to_string()));
    }

    #[test]
    fn exec_unkown_command() {
        let mut cacher = Store::new();
        let res = cacher.exec("FOO bar");
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err().to_string(), "Unknown command");
    }

    #[test]
    fn del_works() {
        let mut cacher = Store::new();
        cacher.set("foo", "bar");
        assert_eq!(cacher.get("foo"), Some("bar".into()));
        cacher.del("foo");
        assert_eq!(cacher.get("foo"), None);
    }

    #[test]
    fn exec_del() {
        let mut cacher = Store::new();
        cacher.set("foo", "bar");
        assert_eq!(cacher.get("foo"), Some("bar".into()));
        let res = cacher.exec("DEL foo");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), None);
        assert_eq!(cacher.get("foo"), None);
    }

    #[test]
    fn del_set() {
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        assert_eq!(
            cacher.smembers("foo"),
            Some(&vec![
                "bar".to_string(),
                "baz".to_string(),
                "qux".to_string()
            ])
        );
        cacher.del("foo");
        assert_eq!(cacher.smembers("foo"), None);
    }

    #[test]
    fn ping() {
        let mut cacher = Store::new();
        let res = cacher.exec("PING");
        assert_eq!(res.unwrap(), Some("PONG".to_string()));
    }
}
