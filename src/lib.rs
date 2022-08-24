pub fn test_function() -> i32 {
    50
}

#[cfg(test)]
mod tests {
    use crate::lib::test_function;

    #[test]
    fn test_function_tester() {
        assert_eq!(50, test_function());
        assert_ne!(213, test_function());
        assert!(50 == test_function());
        assert!(50 != test_function());
    }
}
