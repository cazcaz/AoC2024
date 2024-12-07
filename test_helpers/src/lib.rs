pub fn test_function<T, U, F>(input: Vec<T>, expected: Vec<U>, func: F)
where
    T: std::fmt::Debug + std::cmp::PartialEq,
    U: std::fmt::Debug + std::cmp::PartialEq,
    F: Fn(&T) -> U,
{
    if input.len() != expected.len() {
        panic!("Input and expected were not the same length");
    }
    for (i, input_val) in input.iter().enumerate() {
        assert_eq!(func(input_val), expected[i]);
    }
}
