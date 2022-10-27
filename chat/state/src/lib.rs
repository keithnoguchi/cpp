//! state crate to make the Group state
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Table<T: Clone + Entry>(Mutex<HashMap<Arc<String>, T>>);

pub trait Entry {
    fn new(key: Arc<String>) -> Self;
}

impl<T: Clone + Entry> Default for Table<T> {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

impl<T: Clone + Entry> Table<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &Arc<String>) -> Option<T> {
        self.0.lock().unwrap().get(key).cloned()
    }

    pub fn get_or_create(&self, key: Arc<String>) -> T {
        let mut table = self.0.lock().unwrap();
        let name = key.clone();
        table.entry(key).or_insert_with(|| T::new(name)).clone()
    }
}
