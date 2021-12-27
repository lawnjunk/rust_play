use std::ops::{Add, Sub};
pub mod math;
// mod math_test;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point<T: Copy> {
    x: T,
    y: T,
}

impl<T: Copy + Add<Output = T>> Add for Point<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.y,
            y: self.y + rhs.x,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Point<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.y,
            y: self.y - rhs.x,
        }
    }
}

fn main() {
    let x = math::add(4, 3);
    let y = math::sub(4, 3);

    let a = Point { x: 1, y: 1 };

    let b = Point { x: 0, y: 1 };

    let sum_result = a + b;
    let sub_result = a - b;

    println!("sub_result {:?} sum_result {:?}", sub_result, sum_result);

    println!("add with math::add {:?}", math::add(a, b));
    println!("sub with math::sub {:?}", math::sub(a, b));
    println!("x: {}", x);
    println!("y: {}", y);
}
