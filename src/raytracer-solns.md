# Raytracer Solutions

The full solution is available on [Github](LINK HERE !TODO! ). Feel free to browse through the commit history to see the stages of how I built my solution.

## 1: Images

### Task 1.1

`image` is great, this is dead simple if you find the right bits in the documentation. Learning to use Rust docs is an important skill.

```rust, noplayground
use image::{Rgb, RgbImage};
fn main() {
    let mut buffer = RgbImage::new(256, 256);
    for (_, _, px) in buffer.enumerate_pixels_mut() {
        *px = Rgb([255, 0, 0]);
    }
    buffer.save("render.png").expect("Could not save image");
}
```

This should yield you a big red square. Don't forget to include `image` in your `Cargo.toml`:

```toml
[dependencies]
image = "0.24.1"
```

### Task 1.2

```rust, noplayground
fn main() {
    let width = 400;
    let height = 400;
    let mut buffer = RgbImage::new(256, 256);
    for (i, j, px) in buffer.enumerate_pixels_mut() {
        let r = i as f64 / (width - 1) as f64;
        let g = j as f64 / (height - 1) as f64;
        let b = 0.25;

        *px = Rgb([r, g, b].map(|c| (c * 255.999) as u8))
    }
    buffer.save("render.png").expect("Could not save image");
}
```

We scale the range 0-1 from 0-255 by multiplying by 255.999, as the `as` cast from float to int in Rust rounds down. I also increased the size of the image here to show off our nice gradient a bit better. I changed the size of the image here to demonstrate that it should work for images of any size (not just 256x256, and not just square). Try playing around with different image sizes and gradients.

## 2: Vectors

### 2.1

Our `Vec3` struct with all it's methods:

```rust
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalise(self) -> Self {
        self / self.len()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn to_rgb(self) -> image::Rgb<u8> {
        image::Rgb(
            [self.x, self.y, self.z].map(|c| (c * 255.999) as u8),
        )
    }

    pub fn map<F>(self, mut f: F) -> Vec3
    where
        F: FnMut(f64) -> f64,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}
```

### 2.2

You want the `#[derive]` to look like:

```rust, noplayground
use derive_more::{Add, Constructor, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Add, Div, Mul, Sub, Neg, Constructor)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

Your two hand-written `Mul` impls:

```rust
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| x * self)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
```

### 2.3

Your macro should look as shown in the instructions. Don't worry if it was kinda confusing, macros are hard.

## 3: Rays

### 3.1

You should have a new file `ray.rs`, and a `mod ray;` statement in `main.rs`. In `ray.rs`:

```rust, noplayground
use derive_more::Constructor;

#[derive(Debug, PartialEq, PartialOrd, Clone, Constructor)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
```

### 3.2

Our updated `main` function, with all the camera and geometry definitions:

```rust, noplayground

fn main() {
    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;

    //camera and viewport
    let view_height = 2.0;
    let view_width = view_height * aspect_ratio;
    let focal_length = 1.0;

    //geometry
    let origin: Point = v!(0);
    let horizontal: Vec3 = v!(view_width, 0, 0); //horizontal size vector
    let vertical: Vec3 = v!(0, -view_height, 0); //vertical size vector, negated because we start in the top left and move *down* when rendering
    let top_left: Point = origin - horizontal / 2.0 - vertical / 2.0 - v!(0, 0, focal_length); //the position of the top left corner of our imgae

    let mut buffer = RgbImage::new(img_width, img_height);

    for (i, j, px) in buffer.enumerate_pixels_mut() {
        //pixel coordinates as scalars from 0.0 <= t <= 1.0
        let u = i as f64 / (img_width - 1) as f64;
        let v = j as f64 / (img_height - 1) as f64;

        //the direction of the ray
        //start at top left, then go horizontally scaled by u and vertically by v
        let ray_direction: Vec3 = top_left + u * horizontal + v * vertical - origin;

        //save pixel colour to buffer
        *px = ray::colour(&Ray::new(origin, ray_direction)).to_rgb();
    }
    buffer.save("render.png").expect("Could not save image");
}
```

And the simple green `colour` function, under `ray.rs`:

```rust, noplayground
pub fn colour(ray: &Ray) -> Colour {
   v!(0,1,0)
}
```

### 3.3

Our lerp:

```rust, noplayground
pub fn colour(ray: &Ray) -> Colour {
    let direction = ray.direction.normalise();
    let t = 0.5 * (direction.normalise().y + 1.0); //scale from -1 < y < 1 to  0 < t < 1

    //two colours to blend
    let white: Colour = v!(1);
    let blue: Colour = v!(0.5, 0.7, 1);

    //blend
    blue * t + white * (1.0 - t)
}
```

## 4: Spheres

### 4.1

The entirety of `object.rs` is shown below. Pay careful attention to the quadratic formula in `hit`.

```rust, noplayground
use derive_more::Constructor;

use crate::{ray::Ray, vector::Point};

//a sphere
#[derive(Debug, Constructor)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

//calculate ray-sphere intersection stuff
impl Sphere {
    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }
}
```

This is the condition you want to add to your colour function too

```rust, noplayground
if object::Sphere::new(v!(0, 0, -1), 0.5).hit(ray) {
    return v!(1, 0, 0);
}

```

### 4.2

## 5: Surface Normals & Multiple Objects

## 6: Antialiasing

## 7: Diffuse Materials

## 8: Metal

## 9: Dielectrics

## 10: Positionable Camera

## 11: Defocus Blur