#[cfg(test)]
mod tests{
    use crate::sets::Set;

    #[test]
    fn set_sadd(){
        let mut set = Set::new();
        set.sadd("foo");
        set.sadd("bar");
        set.sadd("baz");
        set.sadd("qux");
        set.sadd("qux");
        assert_eq!(set.sismember("foo"), true);
        assert_eq!(set.sismember("bar"), true);
        assert_eq!(set.sismember("baz"), true);
        assert_eq!(set.sismember("qux"), true);
        assert_eq!(set.sismember("quux"), false);
    }

    #[test]
    fn set_srem(){
        let mut set = Set::new();
        set.sadd("foo");
        set.sadd("bar");
        set.sadd("baz");
        set.sadd("qux");
        set.sadd("qux");
        assert_eq!(set.sismember("foo"), true);
        assert_eq!(set.sismember("bar"), true);
        assert_eq!(set.sismember("baz"), true);
        assert_eq!(set.sismember("qux"), true);
        assert_eq!(set.sismember("quux"), false);
        set.srem("foo");
        assert_eq!(set.sismember("foo"), false);
    }

    #[test]
    fn set_smembers(){
        let mut set = Set::new();
        set.sadd("foo");
        set.sadd("bar");
        set.sadd("baz");
        set.sadd("qux");
        set.sadd("qux");
        assert_eq!(set.smembers(), &vec!["foo", "bar", "baz", "qux"]);
    }
}
