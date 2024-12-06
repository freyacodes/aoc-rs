use std::ops::{Add, Mul, Rem};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point2 {
    x: i32,
    y: i32,
}

pub fn point(x: i32, y: i32) -> Point2 {
    Point2 { x, y }
}

impl Add<&Point2> for &Point2 {
    type Output = Point2;

    fn add(self, rhs: &Point2) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Rem<&Point2> for &Point2 {
    type Output = Point2;

    fn rem(self, rhs: &Point2) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for &Point2 {
    type Output = Point2;

    fn mul(self, rhs: i32) -> Self::Output {
        point(self.x * rhs, self.y * rhs)
    }
}

