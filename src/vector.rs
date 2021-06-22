use std::ops;


#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;
pub type Color = Vec3;


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_center() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn write_color(self) {
        println!(
            "{} {} {}",
            (self.x * 259.99) as i32,
            (self.y * 259.99) as i32,
            (self.z * 259.99) as i32, 
        )
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(v: Self) -> Self {
        v / v.length() 
    }
}

// Utilities

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut rhs: Vec3) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;

        rhs
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, mut rhs: Vec3) -> Self::Output {
        rhs.x /= self;
        rhs.y /= self;
        rhs.z /= self;

        rhs
    }
}