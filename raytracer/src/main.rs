use std::ops::{Add, Div, Index, Mul, Sub};

trait SqrtTrait {
    fn sqrt(self) -> Self;
}

impl SqrtTrait for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

#[derive(Copy, Clone)]
struct Vec<T> {
    x: T,
    y: T,
    z: T,
}

// don't need a second method for commutative addition,
// commutation already works because we're adding floats
impl<T> Add<Vec<T>> for Vec<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec<T>;

    fn add(self, rhs: Vec<T>) -> Vec<T> {
        Vec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub<Vec<T>> for Vec<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec<T>;

    fn sub(self, rhs: Vec<T>) -> Self::Output {
        Vec {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec<T>;

    fn mul(self, _rhs: T) -> Self::Output {
        Vec {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl<T> Div<T> for Vec<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec<T>;

    fn div(self, rhs: T) -> Vec<T> {
        Vec {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Vec<T> {
    fn length(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy + SqrtTrait,
    {
        self.length_squared().sqrt()
    }
}

impl<T> Vec<T> {
    fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

fn main() {
    let v1 = Vec {
        x: 0.58,
        y: 0.58,
        z: 0.58,
    };
    let v2 = Vec {
        x: 0.2,
        y: 0.0,
        z: 0.0,
    };
    println!("v1 + v2: {0}", (v1 + v2).x);
    println!("v1 - v2: {0}", (v1 - v2).x);
    println!("v1 / v2: {0}", (v1 / 2.0).x);
    println!("v1 / v2: {0}", (v1 * 2.0).x);
    println!("squared length of v1: {0}", (v1.length_squared()));
    println!("length of v1: {0}", (v1.length()));
}
