use std::{collections::HashMap, hash::Hash};

pub struct ChainMap<K: Eq + Hash, V> {
    maps: Vec<HashMap<K, V>>,
}

impl<K: Eq + Hash, V> ChainMap<K, V> {
    pub fn new() -> Self {
        Self {
            maps: vec![HashMap::new()],
        }
    }

    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.maps
            .iter()
            .rev()
            .map(|map| map.get(&key))
            .find(|opt| opt.is_some())
            .map(Option::unwrap)
    }

    pub fn set(&mut self, key: K, value: V) -> bool {
        self.maps
            .last_mut()
            .expect("No maps.")
            .insert(key, value)
            .is_some()
    }

    pub fn push_map(&mut self) {
        self.maps.push(HashMap::new());
    }

    pub fn pop_map(&mut self) {
        self.maps.pop();
    }
}
