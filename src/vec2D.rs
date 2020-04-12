use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Mul, Rem, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2D { x, y }
    }
}

impl Vec2D<f64> {
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).abs().sqrt()
    }
    pub fn rotate(self, ang: f64) -> Self {
        let ang_rad = PI * ang / 180.0;
        Self {
            x: (self.x * ang_rad.cos() - self.y * ang_rad.sin()),
            y: (self.x * ang_rad.sin() + self.y * ang_rad.cos()),
        }
    }
    pub fn ang(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl<T: Add<Output = T>> Add<Vec2D<T>> for Vec2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub<Vec2D<T>> for Vec2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Vec2D<T>> for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vec2D<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: scalar.clone() * self.x,
            y: scalar.clone() * self.y,
        }
    }
}
/*
impl<T:Mul<Output = T>> Mul<Vec2D<T>> for T {
    type Output = Vec2D<T>;

    fn mul(self, vec: Vec2D<T>) -> Vec2D<T> {
        Vec2D {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}*/

impl<T: Rem<Output = T>> Rem<Vec2D<T>> for Vec2D<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}
/*
impl<'a, T:Add<Output = T>+Default> Sum<&'a T> for Vec2D<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self { x: T::default(), y: T::default() }, |a, b| Self {
            x: a.x + b.x,
            y: a.y + b.y,
        })
    }
}*/
