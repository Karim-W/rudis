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
    fn multiple_set_and_get(){
        let mut cacher = Store::new();
        cacher.set("foo", "bar");
        assert_eq!(cacher.get("foo"), Some("bar"));
        cacher.set("foo", "baz");
        assert_eq!(cacher.get("foo"), Some("baz"));
        assert_ne!(cacher.get("foo"), Some("bar"));
        assert_eq!(cacher.get("bar"), None);
    }
    #[test]
    fn not_found_handling(){
        let cacher = Store::new();
        assert_eq!(cacher.get("foo"), None);
        assert_eq!(cacher.get("bar"), None);
        assert_eq!(cacher.get("baz"), None);
        assert_eq!(cacher.get("qux"), None);
    }
    #[test]
    fn sadd(){
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
    fn srem(){
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
    fn smembers(){
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        assert_eq!(cacher.smembers("foo"), Some(&vec!["bar".to_string(), "baz".to_string(), "qux".to_string()]));
        assert_eq!(cacher.smembers("bar"), None);
    }

    #[test]
    fn smembers_duplicates(){
        let mut cacher = Store::new();
        cacher.sadd("foo", "bar");
        cacher.sadd("foo", "baz");
        cacher.sadd("foo", "qux");
        cacher.sadd("foo", "qux");
        assert_eq!(cacher.smembers("foo"), Some(&vec!["bar".to_string(), "baz".to_string(), "qux".to_string()]));
        assert_eq!(cacher.smembers("bar"), None);
    }
}
