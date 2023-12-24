use std::ops;

struct Vec {
    x: f64,
    y: f64,
    z: f64,
}

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Vec> for Vec {
    type Output = Vec;

    fn add(self, _rhs: Vec) -> Vec {
        println!("> Foo.add(Bar) was called");
    }
}

// // By reversing the types, we end up implementing non-commutative addition.
// // Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// // This block implements the operation: Bar + Foo = BarFoo
// impl ops::Add<Foo> for Bar {
//     type Output = BarFoo;
//
//     fn add(self, _rhs: Foo) -> BarFoo {
//         println!("> Bar.add(Foo) was called");
//
//         BarFoo
//     }
// }

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT} \n255\n");

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    let mut ir = 0;
    let mut ig = 0;
    let mut ib = 0;
    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {0}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            b = 0.0;

            ir = (255.999 * r) as i32;
            ig = (255.999 * g) as i32;
            ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }
    eprintln!("\rDone.                 \n");
}
