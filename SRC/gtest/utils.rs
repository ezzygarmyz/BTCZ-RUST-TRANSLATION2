#[cfg(test)]
mod tests {
    use crate::utils::{add_numbers, subtract_numbers};

    #[test]
    fn add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
    }

    #[test]
    fn subtract_numbers() {
        assert_eq!(subtract_numbers(5, 3), 2);
    }
}

pub mod utils {
    pub fn add_numbers(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract_numbers(a: i32, b: i32) -> i32 {
        a - b
    }
}
