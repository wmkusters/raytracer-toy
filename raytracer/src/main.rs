use rand::Rng;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

trait Scatterer {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> (bool, Vector, Ray);
}

#[derive(Copy, Clone, Default)]
struct Metal {
    albedo: Vector,
}

impl Scatterer for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> (bool, Vector, Ray) {
        let reflected = r.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        return (true, self.albedo, scattered);
    }
}

#[derive(Copy, Clone, Default)]
struct Lambertian {
    albedo: Vector,
}

impl Scatterer for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> (bool, Vector, Ray) {
        let mut scatter_direction = rec.normal + rand_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        return (true, self.albedo, scattered);
    }
}

fn rand_in_range(min: f64, max: f64) -> f64 {
    let y = rand::thread_rng().gen_range(min..max);
    y as f64
}

fn rand_vector(min: f64, max: f64) -> Vector {
    Vector {
        x: rand_in_range(min, max),
        y: rand_in_range(min, max),
        z: rand_in_range(min, max),
    }
}

fn rand_unit_vector() -> Vector {
    let mut p = rand_vector(-1.0, 1.0);
    loop {
        if p.length_squared() < 1.0 {
            return p.unit_vector();
        }
        p = rand_vector(-1.0, 1.0);
    }
}

fn rand_on_hemisphere(normal: Vector) -> Vector {
    let on_unit_sphere = rand_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Vector,
    pixel00_loc: Vector,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    samples_per_pixel: i32,
    max_depth: u32,
}

fn new_camera(aspect_ratio: f64, image_width: u32, center: Vector) -> Camera {
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // camera
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    let viewport_width: f64 = VIEWPORT_HEIGHT * (image_width as f64 / image_height as f64);

    // horiz/vert viewport edges
    let viewport_u = Vector {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vector {
        x: 0.0,
        y: -VIEWPORT_HEIGHT,
        z: 0.0,
    };

    // horiz/vert delta
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // upper left pixel
    let viewport_upper_left = center
        - Vector {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    const SAMPLES_PER_PIXEL: i32 = 10;
    Camera {
        aspect_ratio,
        image_width,
        image_height,
        center,
        pixel00_loc,
        pixel_delta_u,
        pixel_delta_v,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: 10,
    }
}

impl Camera {
    fn render<T>(self, world: &T)
    where
        T: Hittable,
    {
        println!("P3\n{0} {1} \n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {0}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + ray_color(r, world, self.max_depth);
                }
                write_color(pixel_color, self.samples_per_pixel)
            }
        }
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.center;
        Ray {
            origin: self.center,
            direction: ray_direction,
        }
    }
    fn pixel_sample_square(&self) -> Vector {
        let px = -0.5 + rand_in_range(0.0, 0.99999);
        let py = -0.5 + rand_in_range(0.0, 0.99999);
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }
}

#[derive(Copy, Clone)]
struct Interval {
    min: f64,
    max: f64,
}

impl Default for Interval {
    fn default() -> Interval {
        Interval {
            min: std::f64::INFINITY,
            max: -std::f64::INFINITY,
        }
    }
}

impl Interval {
    fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    fn clamp(self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

struct HitRecord {
    p: Point,
    normal: Vector,
    t: f64,
    front_face: bool,
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Vector::default(),
            normal: Vector::default(),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        };
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> (bool, HitRecord, Option<&Material>) {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut record = HitRecord::default();
        let mut temp_rec: HitRecord;
        let mut mat: Option<&Material> = None;
        let mut new_mat: Option<&Material> = None;

        for obj in &self.objects {
            let h: bool;
            (h, temp_rec, new_mat) = obj.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            );
            if h {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record = temp_rec;
                mat = new_mat;
            }
        }

        (hit_anything, record, mat)
    }
}

impl HittableList {
    fn add(mut self, item: Box<dyn Hittable>) {
        self.objects.push(item);
    }

    fn clear(mut self) {
        self.objects = Vec::new();
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> (bool, HitRecord, Option<&Material>);
}

enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> (bool, Vector, Ray) {
        match self {
            Material::Lambertian(t) => t.scatter(r, rec),
            Material::Metal(t) => t.scatter(r, rec),
        }
    }
}

struct Sphere {
    center: Point,
    radius: f64,
    mat: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> (bool, HitRecord, Option<&Material>) {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let mut record = HitRecord::default();

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (false, record, Some(&self.mat));
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return (false, record, Some(&self.mat));
            }
        }

        record.t = root;
        record.p = r.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);

        return (true, record, Some(&self.mat));
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

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

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

    fn near_zero(self) -> bool {
        const THRESHOLD: f64 = 1e-8;
        self.x < THRESHOLD && self.y < THRESHOLD && self.z < THRESHOLD
    }

    fn reflect(self, n: Vector) -> Vector {
        self - n * (self.dot(n) * 2.0)
    }
}

type Point = Vector;

#[derive(Copy, Clone)]
struct Ray {
    origin: Point,
    direction: Vector,
}

fn ray_color<T>(r: Ray, world: &T, depth: u32) -> Vector
where
    T: Hittable,
{
    if depth == 0 {
        return Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let (h, rec, mat) = world.hit(
        &r,
        Interval {
            min: 0.001,
            max: std::f64::INFINITY,
        },
    );
    if h && mat.is_some() {
        let (s, attenuation, scattered) = mat.unwrap().scatter(&r, &rec);
        if s {
            return ray_color(scattered, world, depth - 1) * attenuation;
        } else {
            return Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
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

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

fn write_color(color: Vector, samples_per_pixel: i32) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval {
        min: 0.0,
        max: 0.999,
    };
    println!(
        "{0} {1} {2}",
        (255.99 * intensity.clamp(r)) as i32,
        (255.99 * intensity.clamp(g)) as i32,
        (255.99 * intensity.clamp(b)) as i32,
    );
}

fn main() {
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let material_ground = Material::Lambertian(Lambertian {
        albedo: Vector {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });
    let material_center = Material::Lambertian(Lambertian {
        albedo: Vector {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });

    let material_left = Material::Metal(Metal {
        albedo: Vector {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    });
    let material_right = Material::Metal(Metal {
        albedo: Vector {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
    });

    // ground
    world.objects.push(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat: material_ground,
    }));

    world.objects.push(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_center,
    }));

    world.objects.push(Box::new(Sphere {
        center: Point {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_left,
    }));

    world.objects.push(Box::new(Sphere {
        center: Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_right,
    }));

    let cam = new_camera(
        16.0 / 9.0,
        1200,
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );

    cam.render(&world);
    eprintln!("\rDone.                 \n");
}
