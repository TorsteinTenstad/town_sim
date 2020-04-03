use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Vec2D<T>{
    pub x: T,
    pub y: T,
}

impl Vec2D::<i32>{
    pub fn magnitude(self) -> f64{
        ((self.x*self.x+self.y*self.y) as f64).abs().sqrt()
    }
}


impl Add<Vec2D::<i32>> for Vec2D::<i32>{
    type Output = Vec2D::<i32>;

    fn add(self, rhs: Vec2D::<i32>) -> Vec2D::<i32>{
        Vec2D::<i32>{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2D::<i32>> for Vec2D::<i32>{
    type Output = Vec2D::<i32>;

    fn sub(self, rhs: Vec2D::<i32>) -> Vec2D::<i32>{
        Vec2D::<i32>{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Vec2D::<i32>> for Vec2D::<i32>{
    fn add_assign(&mut self, rhs: Vec2D::<i32>){
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f64> for Vec2D::<i32>{
    type Output = Vec2D::<i32>;

    fn mul(self, scalar: f64) -> Vec2D::<i32>{
        Vec2D::<i32>{
            x: (scalar * self.x as f64) as i32,
            y: (scalar * self.y as f64) as i32,
        }
    }
}

impl Mul<Vec2D::<i32>> for f64{
    type Output = Vec2D::<i32>;

    fn mul(self, scalar: Vec2D::<i32>) -> Vec2D::<i32>{
        scalar*self
    }
}

impl Rem<Vec2D::<i32>> for Vec2D::<i32>{
    type Output = Vec2D::<i32>;

    fn rem(self, rhs: Vec2D::<i32>) -> Vec2D::<i32>{
        Vec2D::<i32>{
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}