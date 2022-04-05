# Raytracer Solutions

Instead of having the entire solution at each stage written out here, the [Github repo](REPO LINK) has the solutions in, built up from task 1.1 all the way up to the final product in the commit history, with each commit being a single task. You can browse the commit history on Github. I recommend you structure your solution similarly using git.

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

We scale the range 0-1 from 0-255 by multiplying by 255.999, as the `as` cast from float to int in Rust rounds down. I also increased the size of the image here to show off our nice gradient a bit better.

## 2: Vectors

## 3: Rays

## 4: Spheres

## 5: Surface Normals & Multiple Objects

## 6: Antialiasing

## 7: Diffuse Materials

## 8: Metal

## 9: Dielectrics

## 10: Positionable Camera

## 11: Defocus Blur
