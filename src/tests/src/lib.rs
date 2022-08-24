extern crate core;

pub fn test_function(value: i32) -> i32 {
    if value == 50 {
        return 50;
    } else {
        panic!("value isn't 50");
    }
}
#[cfg(test)]
mod tests {
    use crate::test_function;

    #[test]
    fn test_function_tester() {
        assert_eq!(50, test_function(50));
        assert_ne!(213, test_function(50));
        assert!(50 == test_function(50));
        assert!(432 != test_function(50));
    }
    #[test]
    #[should_panic]
    fn test_function_tester_for_panics() {
        test_function(123);
    }
}
