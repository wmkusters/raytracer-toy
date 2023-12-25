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
        T: Mul<Output = T> + Add<Output = T> + Copy + From<f64>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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

#[derive(Copy, Clone)]
struct Ray<T> {
    origin: Point<T>,
    direction: Vec<T>,
}

fn hit_sphere<T>(center: Point<T>, radius: f64, r: &Ray<T>) -> f64
where
    T: Sub<Output = T>
        + Add<Output = T>
        + Mul<Output = T>
        + Copy
        + Display
        + SqrtTrait
        + Div<Output = T>
        + From<f64>,
    f64: From<T>,
{
    let oc = r.origin - center;
    let a = f64::from(r.direction.dot(r.direction));
    let b = f64::from(oc.dot(r.direction) * 2.0.into());
    let c = f64::from(oc.dot(oc)) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn ray_color<T>(r: Ray<T>) -> Vec<f64>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Copy
        + Display
        + SqrtTrait
        + Div<Output = T>
        + From<f64>,
    f64: From<T>,
{
    let t = hit_sphere(
        Vec {
            x: 0.0.into(),
            y: 0.0.into(),
            z: (-1.0).into(),
        },
        0.5,
        &r,
    );
    if t > 0.0 {
        let N = (r.at(t.into())
            - Vec {
                x: 1.0.into(),
                y: 0.0.into(),
                z: 0.0.into(),
            })
        .unit_vector();
        let result = Vec {
            x: f64::from(N.x) + 1.0,
            y: f64::from(N.y) + 1.0,
            z: f64::from(N.z) + 1.0,
        } * 0.5;
        eprintln!("result: {0}", result);
        return result;
    }
    let unit_direction = r.direction.unit_vector();
    let a = f64::from((unit_direction.y + 1.0.into()) * 0.5.into());
    Vec {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - a)
        + Vec {
            x: 0.5.into(),
            y: 0.7.into(),
            z: 1.0.into(),
        } * (a)
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
    eprintln!("{0} {1} {2}", x as i32, y as i32, z as i32,);
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
    const aspect_ratio: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const image_height: u32 = (IMAGE_WIDTH as f64 / aspect_ratio) as u32;

    // camera
    const focal_length: f64 = 1.0;
    const viewport_height: f64 = 2.0;
    const viewport_width: f64 = viewport_height * ((IMAGE_WIDTH / image_height) as f64);
    let camera_center = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // horiz/vert viewport edges
    let viewport_u = Vec {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };

    // horiz/vert delta
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // upper left pixel
    let viewport_upper_left = camera_center
        - Vec {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{IMAGE_WIDTH} {image_height} \n255\n");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {0}", image_height - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = ray_color(r);
            write_color(pixel_color)
        }
    }
    eprintln!("\rDone.                 \n");
    // vec_test()
}
