use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
struct Vec {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// don't need a second method for commutative addition,
// commutation already works because we're adding floats
impl Add<Vec> for Vec {
    type Output = Vec;
    fn add(self, rhs: Vec) -> Vec {
        Vec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec> for Vec {
    type Output = Vec;
    fn sub(self, rhs: Vec) -> Self::Output {
        Vec {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec {
    type Output = Vec;
    fn neg(self) -> Self::Output {
        Vec {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec {
    type Output = Vec;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec {
    type Output = Vec;
    fn div(self, rhs: f64) -> Vec {
        Vec {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

//impl Mul for Vec {
//    type Output = Vec;
//    fn mul(self, rhs: Vec) -> Self::Output {
//        Vec {
//            x: self.x * rhs.x,
//            y: self.y * rhs.y,
//            z: self.z * rhs.z,
//        }
//    }
//}

impl Vec {
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn unit_vector(self) -> Self {
        self / self.length()
    }

    fn dot(self, rhs: Vec) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Vec) -> Vec {
        Vec {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

type Point = Vec;

#[derive(Copy, Clone)]
struct Ray {
    origin: Point,
    direction: Vec,
}

fn hit_sphere(center: Point, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Vec {
    let t = hit_sphere(
        Vec {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        &r,
    );
    if t > 0.0 {
        let N = (r.at(t)
            - Vec {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            })
        .unit_vector();
        let result = Vec {
            x: N.x + 1.0,
            y: N.y + 1.0,
            z: N.z + 1.0,
        } * 0.5;
        return result;
    }
    let unit_direction = r.direction.unit_vector();
    let a = (unit_direction.y + 1.0) * 0.5;
    Vec {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - a)
        + Vec {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * (a)
}

impl Ray {
    fn at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

fn write_color(color: Vec) {
    let output = color * 255.999;
    println!(
        "{0} {1} {2}",
        output.x as i32, output.y as i32, output.z as i32,
    );
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // camera
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let camera_center = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // horiz/vert viewport edges
    let viewport_u = Vec {
        x: VIEWPORT_WIDTH,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec {
        x: 0.0,
        y: -VIEWPORT_HEIGHT,
        z: 0.0,
    };

    // horiz/vert delta
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // upper left pixel
    let viewport_upper_left = camera_center
        - Vec {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT} \n255\n");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {0}", IMAGE_HEIGHT - j);
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
}
