use std::ops::{Add, Sub};

pub fn add<T: Copy + Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn sub<T>(a: T, b: T) -> T
where
    T: Copy + Sub<T, Output = T>,
{
    a - b
}

#[cfg(test)]
mod math_test {
    use crate::math;
    #[test]
    fn should_add() {
        assert_eq!(math::add(2, 2), 4);
        assert_eq!(math::add(2.0, 2.0), 4.0);
    }

    #[test]
    fn should_sub() {
        assert_eq!(math::sub(2, 2), 0);
        assert_eq!(math::sub(2.0, 2.0), 0.0);
    }
}
