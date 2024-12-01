#[cfg(test)]
mod tests {
    use crate::memusage::dynamic_usage;

    #[test]
    fn vector_usage() {
        let vec = vec![0; 100];
        assert_eq!(dynamic_usage(&vec), 400);
    }
}

pub mod memusage {
    pub fn dynamic_usage<T>(vec: &Vec<T>) -> usize {
        vec.len() * std::mem::size_of::<T>()
    }
}
