# Raytracer Project

For those of you not familiar with raytracing, it's a 3d graphics rendering technique that works by modelling light rays that's used becoming more common as a technique in modern gaming (thanks to NVIDIA). Ray tracing is a bit of an umbrella term, but what we're gonna build is technically a path tracer, and a fairly general one.

This tutorial is adapted from the excellent [_Ray Tracing in One Weekend_](https://raytracing.github.io/), amd also utilises all of the excellent illustrations from there. I've rewritten it from C++ to Rust, and also added a few other bits to hopefully make it more interesting and explore a few more bits of Rust. This is only an adaptation of the first book, so if you get to the end of this and want to explore more, the next two books are certainly worth a read, though you'll have to convert the C++ to Rust yourself (or do it in C++, which despite all it's problems is still widely used and a good skill to have).

There's a fair amount of vector maths involved here but don't let that intimidate you. I'll try to explain it all well enough that you don't need a maths degree to follow whats going on.

Also, unlike the original book and like the previous project, I'm not going to give you the code snippets as we go. Feel free to take a look at the solutions if you get stuck, but try to solve the tasks yourself as you'll find it much more rewarding. Remember to make use of your resources!

## 1: Images

What does a renderer render? Well... pictures. An image of a scene. So we're going to need some way to output an image in Rust. We're going to take advantage of the excellent crates.io ecosystem here and use a crate called [image](https://github.com/image-rs/image) that does pretty much exactly what it says on the tin: provide tools for working with images. Have a look over the docs over on [docs.rs](https://docs.rs/image/latest/image/) and have a think about how you might go about creating a new image.

We don't need to support any fancy encoding or anything for our raytracer, we just want each pixel to be comprised of 3 bytes: the good old `(r, g, b)`.

### Task 1.1

I'm assuming you've already created a new cargo project and added `image` to your dependencies in `Cargo.toml`. In your main function, write some code to generate an image and save it to disk. The basic steps are something like:

- Create a new image buffer to hold our pixels ([docs](https://docs.rs/image/latest/image/struct.ImageBuffer.html)). 256x256 should be good to start with.
- Iterate over each pixel ([docs](https://docs.rs/image/latest/image/struct.ImageBuffer.html#method.enumerate_pixels_mut))
  - Modify each pixel, setting it to a single colour (red) to start with ([docs](https://docs.rs/image/latest/image/struct.Rgb.html))
- Save the buffer to a file ([docs](https://docs.rs/image/latest/image/struct.ImageBuffer.html#method.save))

Your image should be saved to disk, and look like this:

![](./img/1-1.png)

### Task 1.2

We're gonna extend the code from above to output something a bit nicer. From here on out, I'm going to talk about RGB values as being floats from 0.0 to 1.0. Converting them to bytes can be done just before you write to the pixel. I'm also going to refer to `i` and `j` as the coordinates of each pixel, where `i` is the offset in columns from the top-left corner, and `j` is the offset in rows (if you're not already using an iterator that does this for you, try find it in the `image` documentation).

For each pixel:

- Scale `i` to the range 0.0-1.0 based upon the image's width. This will be your `r` value.
- Scale `j` to the range 0.0-1.0 based upon the image's height. This will be your `g` value.
- Fix `b` at 0.25
- Convert your rgb values to bytes
- Write your pixel to the buffer.

Red will fade from 0 to 1 left to right, and green will fade in from top to bottom. You should get a nice gradient like this:

![](./img/1-2.png)

This is a of the graphics "Hello World", because once we have an image we can do what we want with it.

## 2: Vectors

Almost all graphics programs have some data structures for storing geometric vectors and colors. In many systems these vectors are 4D (3D plus a homogeneous coordinate for geometry, and RGB plus an alpha transparency channel for colors). For our purposes, three coordinates suffices. We’ll use the same struct `Vec3` for colors, locations, directions, offsets, whatever. Some people don’t like this because it doesn’t prevent you from doing something silly, like adding a color to a location. They have a good point, and we could enforce this through Rust's type system, but we're going to not for now because it adds a lot of complexity. We will create some type aliases `Colour` and `Point`, though, to make our types a little more descriptive.

### Task 2.1

Our `Vec3` will require a few methods to make it useful in graphics applications:

- [Dot](https://www.mathsisfun.com/algebra/vectors-dot-product.html) and [cross](https://www.mathsisfun.com/algebra/vectors-cross-product.html) products
- A `len()` method, to get it's magnitude
- A `normalise()` method, to convert a vector to a vector with the same direction but magnitude 1.
- A `to_rgb()` method that converts a vector with all all 0.0-1.0 values to an `image::Rgb`.
- A `map()` method, that applies a function to each element of the vector, consuming it and returning a new vector.

Create a new `vector.rs` file, and include it in the module tree with a `mod vector;` statement in main. Then create a simple struct, `Vec3`, with three `f64` fields: x, y, z. Then, implement all these methods on it. Start with `dot()` and `len()`, then try `cross()`. Do `map()` next, as you can use it to then implement `to_rgb()` and `normalise()`. Look at the docs for [std::array::map](https://doc.rust-lang.org/std/primitive.array.html#method.map) for help with your map implementation, you want to take some function as an argument, and apply it to all 3 elements in your vector.

Add two type aliases `pub type Colour = Vec3` and `pub type Point = Vec3` too, You can add any other general vector methods you think might come in handy too.

### Task 2.2

We'll also want to overload some operators. Operator overloading allows operators to be overloaded to work on custom types, which is done in Rust by implementing the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) traits. You want to be able to:

- Add two vectors
- Subtract two vectors
- Multiply a vector with a float
- Divide a vector by a float
- Negate a vector
- Multiply a vector element-wise by another vector

Implementing all of these means a lot of boilerplate, but we can draft in another crate to help us: [`derive_more`](https://github.com/JelteF/derive_more), which extends the `#[derive]` macros we're familiar with by allowing us to derive more traits, including operators. Add it to your `Cargo.toml` and have a look at the docs to see how to use it. Add derives for `Add`, `Sub`, `Mul`, `Div`, and `Neg`. You can also derive a `Constructor`! Add `Debug`, `PartialEq`, and `PartialOrd` while you're at it too.

Our vector is only 24 bytes, so can be considered cheap enough to derive `Copy` and `Clone` for it too. Remember that this disregards move semantics for the vector to let the compiler automatically make copies of it where needed.

`derive_more` isn't perfect, so we need to add a few operator overloads manually. `Mul<f64> for Vec3` is derived for us, which gives us `mul(Vec3, f64)`, but not the other way round (Rust does not assume that multiplication is commutative when implementing these traits). We can get the other way round with an `impl Mul<Vec3> for f64`, so we technically implement the trait again for the `f64` type. Take a look at the docs for [`std::ops::Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) to work out how to do this.

There's also one or two cases where we want to multiply a vector by another vector element-wise. Add another `Mul` implementation for `Vec3` to do this.

### Task 2.3

We're gonna take a quick foray down the rabbit hole that is Rust macros to create a ~~dirty hack~~ shorthand for initialising new vectors, since we're going to be doing an awful lot of it. I recommend having a read through [this blog post](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/), and some of the [Rust by Example](https://doc.rust-lang.org/rust-by-example/macros.html) chapter, then I'll walk you through it.

Declarative macros are just functions that operate on syntax by pattern matching. Our macro is declared using `macro_rules!`, and we'll call it `v!` (because its for vectors).

```rust
macro_rules v! {
    //patterns
}
```

Patterns are declared using syntax similar to `match`: `() => {}`. The macro matches on the pattern in the parentheses, and then the macro will expand to the code in the braces. In the parentheses goes the arguments to the macrom, which are Rust syntax items, specified like `$x: ty`, where `$x` is the name of the token, and `ty` is the type of the token. Theres a few kinds of tokens, but we'll just use `expr` for now, which covers any expression.

```rust
macro_rules v! {
    ($x: expr) => {
        Vec3::new($x, $x, $x)
    }
}
```

The macro above takes a single expression as an argument, and replaces it with a call to `Vec3::new` with the same expression as all 3 arguments. A call to `v!(1)` will be expanded to `Vec3::new(1, 1, 1)`. We don't have to just use numbers though, the macro can be called on _any valid expression_.

We're going to add another pattern too to create a vector with three different arguments. The macro will pattern match on the two sets of arguments, and expand the one that matches. If no patterns match, the code won't compile.

```rust
macro_rules! v {
    ($x: expr, $y: expr, $z: expr) => {
        Vec3::new($x, $y, $z)
    };
    ($x: expr) => {
        Vec3::new($x, $x, $x)
    };
}
```

We'll add another neat little trick too. The `f64::from` uses the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) trait to accept any value that can be easily converted to an f64, and returns the float. For example, we can do `f64::from(0_u8)`, `f64::from(0_i32)` and `f64::from(0.0_f32)`, and get `0.0_f64` from all of them. Using this in our macro lets it be a little more flexible.

```rust
#[macro_export]
macro_rules! v {
    ($x:expr, $y: expr, $z: expr) => {
        Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
    ($x:expr) => {
        Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };
}
```

The `#[macro_export]` annotation at the top tells Rust to export our macro at the crate root so other modules in our crate can use it with `use crate::v`. Exporting/using macros is a bit funky in Rust, but don't worry about it too much for now.

## 3: Rays

All ray tracers need some data type to represent rays. Think of a ray of a function $\mathbf P(t) = \mathbf A + t\mathbf b$.

- $\mathbf P$ is a position on a line in 3 dimensions
- $\mathbf A$ is the ray origin
- $\mathbf B$ is the direction the ray is pointing

Using this, you can plug in a different parameter `t` to get a position anywhere on the line/ray.

![](https://raytracing.github.io/images/fig-1.02-lerp.jpg)

### 3.1

Create a new `ray` module. Create a new struct in it that stores the origin `Point` and direction `Vec3` of a ray, and add a method `Ray::at(&self, t: f64) -> Point` that returns the point in 3d space that is `t` units along the ray. Either create or derive a constructor for your `Ray` too.

### 3.2

Now we have rays, we can finally trace some. The basic concept is that the ray tracer casts rays from a "camera" and through each pixel, calculating the colour of each pixel. Like light, but in reverse. We'll start with a simple camera defined with a few basic parameters, and a `ray::colour` function that computes the colour of a ray.

Our basic image will use a 16:9 aspect ratio, because it's common, and because with a square image its easy to introduce bugs by accidentally transposing `x` and `y`. We'll also set up a virtual viewport that our rays will pass through into our scene, that will be two units wide and one unit away from the camera. The camera will be at $(0, 0, 0)$, with the `y` axis going up and `x` to the left. To respect the convention of a right handed coordinate system, into the screen is the negative z-axis. We will traverse the screen from the upper left hand corner, and use two offset vectors $\textbf u$ and $\textbf v$ along the screen sides to move the ray across the screen.

![](https://raytracing.github.io/images/fig-1.03-cam-geom.jpg)

- Define your aspect ratio as `16/9`, your width as 400, and your height accordingly.
- The viewport height should be `2.0`, and width should be set accordingly in line with the aspect ratio.
- The focal length should be `1.0`
- Looking at the diagram above, we can see that the top left corner lies at $\textbf O - \textbf x /2 + \textbf y/2 - \textbf f$
  - $\textbf x$ and $\textbf y$ are your image height and width vectors
  - $\textbf f$ is your focal length vector

Write a `colour(&Ray) -> Colour` function that just always returns `v!(0, 1.0, 0)` for now, we'll add a nice pattern later. Update your loop in your `main` function to calculate the direction vector of the ray to cast on each iteration based on `i` and `j`, and then create a ray starting at the origin and going into the pixel. You can do this by scaling your pixel coordinate from 0 to 1, and then multiplying by your height and width vectors. Colour your ray and save the pixel value to the buffer calling `Vec3::to_rgb` to convert your colour from 0-1 from 0-255.

You should get a nice green rectangle. I appreciate theres a lot going on there, so ask for help or take a look at the solutions if you're not sure.

### 3.3

To make the background for our raytraced image, we're gonna add add a nice blue-white blend. In your colour function, add code to normalise the ray's direction vector, then scale it from $0 \leq t \leq 1$ from $-1 \leq t \leq 1$. We're then gonna do a neat graphics trick called a lerp, or linear interpolation, where we blend two colours: `blended_value = (1-t) * start_value + t * end_value`. Use `v!(1)` for your starting colour, `v!(0.5, 0.7, 1.0)` for your end colour, and blend based upon the y coordinate. You should end up with something like:

![](./img/3-3.png)

If your colours don't look similar, or it's upside down, check your geometry is correct.

## 4: Spheres

Spheres are often used in raytracers because its fairly easy to work out if a ray has hit one or not. The equation for a sphere

### 4.1

the image

### 4.3

rayon

### 4.4

indicatif

## 5: Surface Normals & Multiple Objects

## 6: Antialiasing

## 7: Diffuse Materials

## 8: Metal

## 9: Dielectrics

## 10: Positionable Camera

## 11: Defocus Blur

## What next?

```

```
