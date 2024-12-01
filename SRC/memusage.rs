use std::collections::{HashMap, HashSet};
use std::mem;

/// Calculates the memory usage of a `Vec<T>`
pub fn mem_usage_vec<T>(vec: &Vec<T>) -> usize {
    vec.capacity() * mem::size_of::<T>()
}

/// Calculates the memory usage of a `HashMap<K, V>`
pub fn mem_usage_hashmap<K, V>(map: &HashMap<K, V>) -> usize {
    map.capacity() * (mem::size_of::<K>() + mem::size_of::<V>())
}

/// Calculates the memory usage of a `HashSet<T>`
pub fn mem_usage_hashset<T>(set: &HashSet<T>) -> usize {
    set.capacity() * mem::size_of::<T>()
}

/// Calculates the memory usage of a string
pub fn mem_usage_string(s: &String) -> usize {
    s.capacity()
}

/// Estimates memory usage of a custom object
pub fn mem_usage_custom<T>(object: &T) -> usize {
    mem::size_of::<T>()
}

/// Estimates memory usage of a slice
pub fn mem_usage_slice<T>(slice: &[T]) -> usize {
    slice.len() * mem::size_of::<T>()
}
