use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Default)]
struct HitRecord {
    p: Point,
    normal: Vector,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(self, r: &Ray, outward_normal: Vector) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(self, r: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let hit_anything = false;
        let closest_so_far = ray_tmax;

        for obj in &self.objects {
            if obj.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record = &mut temp_rec;
            }
        }
        hit_anything
    }
}

impl HittableList {
    fn add(self, item: Box<dyn Hittable>) {
        self.objects.push(item);
    }

    fn clear(self) {
        self.objects = Vec::new();
    }
}

trait Hittable {
    fn hit(self, r: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Point,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(self, r: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        record.t = root;
        record.p = r.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);

        return true;
    }
}

#[derive(Copy, Clone, Default)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// don't need a second method for commutative addition,
// commutation already works because we're adding floats
impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Vector {
        Vector {
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

impl Vector {
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn unit_vector(self) -> Self {
        self / self.length()
    }

    fn dot(self, rhs: Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

type Point = Vector;

#[derive(Copy, Clone)]
struct Ray {
    origin: Point,
    direction: Vector,
}

fn ray_color(r: Ray, world: Box<dyn Hittable>) -> Vector {
    let rec = HitRecord::default();
    if world.hit(&r, 0.0, std::f64::INFINITY, &mut rec) {
        return (rec.normal
            + Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            })
            * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let a = (unit_direction.y + 1.0) * 0.5;
    Vector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - a)
        + Vector {
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

fn write_color(color: Vector) {
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

    let world = HittableList {
        objects: Vec::new(),
    };

    world.add(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

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
    let viewport_u = Vector {
        x: VIEWPORT_WIDTH,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vector {
        x: 0.0,
        y: -VIEWPORT_HEIGHT,
        z: 0.0,
    };

    // horiz/vert delta
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // upper left pixel
    let viewport_upper_left = camera_center
        - Vector {
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

            let pixel_color = ray_color(r, Box::new(world));
            write_color(pixel_color)
        }
    }
    eprintln!("\rDone.                 \n");
}
