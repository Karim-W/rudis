use std::collections::HashMap;
pub mod tests;

pub struct Set<T>{
    data: HashMap<T, usize>,
    slice : Vec<T>,
}

impl Set<String> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            slice: Vec::new(),
        }
    }

    pub fn sadd(&mut self, value: &str) {
        if !self.data.contains_key(value) {
            self.data.insert(value.to_string(), self.slice.len());
            self.slice.push(value.to_string());
        }
    }

    pub fn smembers(&self) -> &Vec<String> {
        &self.slice
    }

    pub fn sismember(&self, value: &str) -> bool {
        self.data.contains_key(value)
    }

    pub fn srem(&mut self, value: &str) {
        match self.data.get(value) {
            Some(index) => {
                self.slice.remove(*index);
                self.data.remove(value);
            }
            None => {}
        }
    }
}


