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

type Point<T> = Vec<T>;

struct Ray<T> {
    origin: Point<T>,
    direction: Vec<T>,
}

impl<T> Ray<T> {
    fn at(self, t: T) -> Point<T>
    where
        T: Mul<Output = T> + Copy + From<f64>,
        Vec<T>: Add<Output = Vec<T>>,
    {
        self.origin + self.direction * t
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

fn vec_test() {
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
    let r1 = Ray {
        origin: Point {
            x: 1.1,
            y: 1.2,
            z: 1.3,
        },
        direction: Vec {
            x: 1.1,
            y: 1.2,
            z: 1.3,
        },
    };
    println!("ray: {0}", r1.at(4.0));
}

fn main() {
    /*
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT} \n255\n");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {0}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let color = Vec { x: r, y: g, z: b };
            write_color(color)
        }
    }
    eprintln!("\rDone.                 \n");
    */
    vec_test()
}
