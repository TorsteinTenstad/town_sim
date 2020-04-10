use std::ops::*;
use std::f64::consts::PI;
use std::f64::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2D<T>{
    pub x: T,
    pub y: T,
}

impl Vec2D::<i32>{
    pub fn magnitude(self) -> f64{
        ((self.x*self.x+self.y*self.y) as f64).abs().sqrt()
    }
    pub fn rotate(self, ang: f64) -> Self{
        let ang_rad = PI*ang/180.0;
        Self{x: (self.x as f64*ang_rad.cos() - self.y as f64*ang_rad.sin()) as i32,
             y: (self.x as f64*ang_rad.sin() + self.y as f64*ang_rad.cos()) as i32}
    }
    pub fn ang(self) -> f64{
        (self.y as f64).atan2(self.x as f64)
    }
}


impl<T:Add<Output = T>> Add<Vec2D::<T>> for Vec2D::<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self{
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T:Sub<Output = T>> Sub<Vec2D::<T>> for Vec2D::<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self{
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T:AddAssign> AddAssign<Vec2D::<T>> for Vec2D::<T>{
    fn add_assign(&mut self, rhs: Self){
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f64> for Vec2D::<i32>{
    type Output = Vec2D::<i32>;

    fn mul(self, scalar: f64) -> Vec2D::<i32>{
        Self{
            x: (scalar * self.x as f64) as i32,
            y: (scalar * self.y as f64) as i32,
        }
    }
}

impl Mul<Vec2D::<i32>> for f64{
    type Output = Vec2D::<i32>;

    fn mul(self, vec: Vec2D::<i32>) -> Vec2D::<i32>{
        vec*self
    }
}

impl<T:Rem<Output = T>> Rem<Vec2D::<T>> for Vec2D::<T>{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self{
        Self{
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}