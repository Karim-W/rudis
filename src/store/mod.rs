use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
pub struct Store {
    mtx: Arc<Mutex<i32>>,
    data: HashMap<String, String>,
    sets: HashMap<String, Vec<String>>,
}

enum Command {
    Get(String),
    Set(String, String),
    Sadd(String, String),
    Smembers(String),
    Sismember(String, String),
    Srem(String, String),
    None,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Command::Get(s.to_string()),
            "SET" => Command::Set(s.to_string(), s.to_string()),
            "SADD" => Command::Sadd(s.to_string(), s.to_string()),
            "SMEMBERS" => Command::Smembers(s.to_string()),
            "SISMEMBER" => Command::Sismember(s.to_string(), s.to_string()),
            "SREM" => Command::Srem(s.to_string(), s.to_string()),
            _ => Command::None,
        }
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            mtx: Arc::new(Mutex::new(0)),
            data: HashMap::new(),
            sets: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&str> {
        let _lock = self.mtx.lock().unwrap();
        self.data.get(key).map(|s| s.as_str())
    }

    fn set(&mut self, key: &str, value: &str) {
        let _lock = self.mtx.lock().unwrap();
        self.data.insert(key.to_string(), value.to_string());
    }

    fn sadd(&mut self, key: &str, value: &str) {
        let _lock = self.mtx.lock().unwrap();
        let set = self.sets.entry(key.to_string()).or_insert(Vec::new());
        set.push(value.to_string());
    }

    fn smembers(&self, key: &str) -> Option<&Vec<String>> {
        let _lock = self.mtx.lock().unwrap();
        self.sets.get(key)
    }

    fn sismember(&self, key: &str, value: &str) -> bool {
        let _lock = self.mtx.lock().unwrap();
        match self.sets.get(key) {
            Some(set) => set.contains(&value.to_string()),
            None => false,
        }
    }

    fn srem(&mut self, key: &str, value: &str) {
        let _lock = self.mtx.lock().unwrap();
        match self.sets.get_mut(key) {
            Some(set) => {
                let index = set.iter().position(|x| x == value);
                match index {
                    Some(index) => {
                        set.remove(index);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    pub fn exec(&mut self, query: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let split: Vec<&str> = query.split(' ').collect();
        let command = split[0];
        let key = split[1];
        match command.into() {
            Command::Get(_) => {
                let value = self.get(key);
                match value {
                    Some(value) => Ok(Some(value.to_string())),
                    None => Ok(None),
                }
            }
            Command::Set(_, _) => {
                let value = split[2];
                self.set(key, value);
                Ok(None)
            }
            Command::Sadd(_, _) => {
                let value = split[2];
                self.sadd(key, value);
                Ok(None)
            }
            Command::Smembers(_) => {
                let value = self.smembers(key);
                match value {
                    Some(value) => Ok(Some(format!("{:?}", value))),
                    None => Ok(None),
                }
            }
            Command::Sismember(_, _) => {
                let value = split[2];
                let is_member = self.sismember(key, value);
                Ok(Some(format!("{}", is_member)))
            }
            Command::Srem(_, _) => {
                let value = split[2];
                self.srem(key, value);
                Ok(None)
            }
            Command::None => Err("Invalid command".into()),
        }
    }
}