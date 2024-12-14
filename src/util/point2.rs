use std::ops::{Add, Mul, Sub};
use lazy_static::lazy_static;

pub fn point(x: i32, y: i32) -> Point2 {
    Point2 { x, y }
}

lazy_static! {
    pub static ref CARDINALS_AND_DIAGONALS: Vec<Point2> = vec![
        point(1, 0),  // East
        point(1, -1), // Southeast
        point(0, -1), // South
        point(-1, -1),// Southwest
        point(-1, 0), // West
        point(-1, 1), // Northwest
        point(0, 1),  // North
        point(1, 1),  // Northeast
    ];
    
    pub static ref CARDINALS: Vec<Point2> = vec![
        point(1, 0),  // East
        point(0, -1), // South
        point(-1, 0), // West
        point(0, 1),  // North
    ];
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point2 {
    x: i32,
    y: i32,
}

impl Add<&Point2> for &Point2 {
    type Output = Point2;

    fn add(self, rhs: &Point2) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<&Point2> for &Point2 {
    type Output = Point2;

    fn sub(self, rhs: &Point2) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for &Point2 {
    type Output = Point2;

    fn mul(self, rhs: i32) -> Self::Output {
        point(self.x * rhs, self.y * rhs)
    }
}

