use crate::commands::Command;
use crate::sets::Set;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
pub mod tests;
pub struct Store {
    mtx: Arc<Mutex<i32>>,
    data: HashMap<String, String>,
    sets: HashMap<String, Set<String>>,
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
        println!("set {} {}", key, value);
        let _lock = self.mtx.lock().unwrap();
        self.data.insert(key.to_string(), value.to_string());
    }

    fn del(&mut self, key: &str) {
        let _lock = self.mtx.lock().unwrap();
        if self.sets.contains_key(key) {
            self.sets.remove(key);
        }
        if self.data.contains_key(key) {
            self.data.remove(key);
        }
    }

    fn sadd(&mut self, key: &str, value: &str) {
        let _lock = self.mtx.lock().unwrap();
        let set = self.sets.entry(key.to_string()).or_insert(Set::new());
        set.sadd(value);
    }

    fn smembers(&self, key: &str) -> Option<&Vec<String>> {
        let _lock = self.mtx.lock().unwrap();
        self.sets.get(key).map(|s| s.smembers()).or(None)
    }

    fn sismember(&self, key: &str, value: &str) -> bool {
        let _lock = self.mtx.lock().unwrap();
        match self.sets.get(key) {
            Some(set) => set.sismember(value),
            None => false,
        }
    }

    fn srem(&mut self, key: &str, value: &str) {
        let _lock = self.mtx.lock().unwrap();
        match self.sets.get_mut(key) {
            Some(set) => {
                set.srem(value);
            }
            None => {}
        }
    }

    fn ping(&self) -> String {
        "PONG".to_string()
    }

    pub fn exec(&mut self, query: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let split: Vec<&str> = query.split(' ').collect();
        println!("{:?}", split);
        if split.len() == 0 {
            return Err("Invalid command".into());
        }
        let command = split[0];
        let key = if split.len() > 1 { split[1] } else { "" };
        match command.into() {
            Command::Ping => Ok(Some(self.ping())),
            Command::Get(_) => {
                println!("GET COMMAND INCOMING");
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
            Command::Del(_) => {
                self.del(key);
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
            Command::None => Err("Unknown command".into()),
        }
    }
}
