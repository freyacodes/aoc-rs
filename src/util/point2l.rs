use std::ops::{Add, Mul, Sub};

pub fn point(x: i64, y: i64) -> Point2L {
    Point2L { x, y }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point2L {
    pub x: i64,
    pub y: i64,
}

impl Add<&Point2L> for &Point2L {
    type Output = Point2L;

    fn add(self, rhs: &Point2L) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<&Point2L> for &Point2L {
    type Output = Point2L;

    fn sub(self, rhs: &Point2L) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i64> for &Point2L {
    type Output = Point2L;

    fn mul(self, rhs: i64) -> Self::Output {
        point(self.x * rhs, self.y * rhs)
    }
}

