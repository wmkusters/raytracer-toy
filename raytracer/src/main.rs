use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl<T> Display for Vec<T>
where
    T: Display + From<f64>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// don't need a second method for commutative addition,
// commutation already works because we're adding floats
impl<T> Add<Vec<T>> for Vec<T>
where
    T: Add<Output = T> + Copy + From<f64>,
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
    T: Sub<Output = T> + Copy + From<f64>,
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

impl<T> Neg for Vec<T>
where
    T: Neg<Output = T> + Copy + From<f64>,
{
    type Output = Vec<T>;

    fn neg(self) -> Self::Output {
        Vec {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Mul<T> for Vec<T>
where
    T: Mul<Output = T> + Copy + From<f64>,
{
    type Output = Vec<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Vec<T>
where
    T: Div<Output = T> + Copy + From<f64>,
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

impl<T> Mul<Vec<T>> for Vec<T>
where
    T: Mul<Output = T> + Copy + From<f64>,
{
    type Output = Vec<T>;

    fn mul(self, rhs: Vec<T>) -> Self::Output {
        Vec {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Vec<T>
where
    T: From<f64>,
{
    fn length(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy + SqrtTrait + From<f64>,
    {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy + From<f64>,
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn unit_vector(self) -> Self
    where
        T: Add<Output = T> + Mul<Output = T> + Copy + SqrtTrait + Div<Output = T> + From<f64>,
    {
        self / self.length()
    }

    fn dot(self, rhs: Vec<T>) -> T
    where
        T: Add<Output = T> + Copy + From<f64>,
    {
        self.x + rhs.x + self.y + rhs.y + self.z + rhs.z
    }

    fn cross(self, rhs: Vec<T>) -> Vec<T>
    where
        T: Mul<Output = T> + Sub<Output = T> + Copy + From<f64>,
    {
        Vec {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

fn write_color<T>(color: Vec<T>)
where
    T: Mul<Output = T> + Copy + From<f64>,
    f64: From<T>,
{
    let output = color * T::from(255.99);
    let x = f64::from(output.x);
    let y = f64::from(output.y);
    let z = f64::from(output.z);
    println!("{0} {1} {2}", x as i32, y as i32, z as i32,);
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
    println!("v1: {0}", (v1));
    println!("-v1: {0}", (-v1));
    println!("v1 / 2.0: {0}", (v1 / 2.0));
    println!("v1 * 2.0: {0}", (v1 * 2.0));
    println!("v1 + v2: {0}", (v1 + v2));
    println!("v1 - v2: {0}", (v1 - v2));
    println!("v1 * v2: {0}", (v1 * v2));
    println!("squared length of v1: {0}", (v1.length_squared()));
    println!("length of v1: {0}", (v1.length()));
    println!("unit of v1: {0}", (v1.unit_vector()));
    println!("v1.dot(v2): {0}", (v1.dot(v2)));
    println!("v1.cross(v2): {0}", (v1.cross(v2)));
    write_color(v1);
    write_color(v2);
}
