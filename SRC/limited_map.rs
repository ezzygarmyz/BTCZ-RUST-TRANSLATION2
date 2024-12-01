use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

/// A map with a limited size. When the size limit is exceeded, the oldest elements are removed.
pub struct LimitedMap<K, V>
where
    K: Eq + Hash + Clone,
{
    map: HashMap<K, V>,
    keys: VecDeque<K>,
    max_size: usize,
}

impl<K, V> LimitedMap<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates a new LimitedMap with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        LimitedMap {
            map: HashMap::new(),
            keys: VecDeque::new(),
            max_size,
        }
    }

    /// Inserts a key-value pair into the map. If the map exceeds its size limit, the oldest entry is removed.
    pub fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.keys.retain(|k| k != &key);
        }

        self.keys.push_back(key.clone());
        self.map.insert(key, value);

        if self.keys.len() > self.max_size {
            if let Some(oldest_key) = self.keys.pop_front() {
                self.map.remove(&oldest_key);
            }
        }
    }

    /// Retrieves a value associated with a key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /// Checks if the map contains a specific key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Returns the current size of the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.map.clear();
        self.keys.clear();
    }

    /// Returns the maximum size of the map.
    pub fn max_size(&self) -> usize {
        self.max_size
    }
}
