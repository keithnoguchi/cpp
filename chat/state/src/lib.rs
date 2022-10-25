//! state crate to make the Group state
use std::collections::HashMap;
use std::sync::Arc;

pub struct Table<T: Clone + Entry> {
    table: HashMap<Arc<String>, T>,
}

pub trait Entry {
    fn new() -> Self;
}

impl<T: Clone + Entry> Default for Table<T> {
    fn default() -> Self {
        let table = HashMap::new();
        Self { table }
    }
}

impl<T: Clone + Entry> Table<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &Arc<String>) -> Option<&T> {
        self.table.get(key)
    }

    pub fn get_or_create(&mut self, key: Arc<String>) -> &mut T {
        self.table.entry(key).or_insert_with(T::new)
    }
}
