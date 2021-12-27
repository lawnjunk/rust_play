#[cfg(test)]
mod wat_math_test {
    use crate::math;
    #[test]
    fn cool() {
        println!("coool");
        assert_eq!(math::sub(1, 1), 0);
    }
}
