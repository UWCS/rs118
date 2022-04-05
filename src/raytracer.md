# Raytracer Project

For those of you not familiar with raytracing, it's a 3d graphics rendering technique that works by modelling light rays that's used becoming more common as a technique in modern gaming (thanks to NVIDIA). Ray tracing is a bit of an umbrella term, but what we're gonna build is technically a path tracer, and a fairly general one.

This tutorial is adapted from the excellent [_Ray Tracing in One Weekend_](https://raytracing.github.io/). I've rewritten it from C++ to Rust, and also added a few other bits to hopefully make it more interesting and explore a few more bits of Rust. This is only an adaptation of the first book, so if you get to the end of this and want to explore more, the next two books are certainly worth a read, though you'll have to convert the C++ to Rust yourself (or do it in C++, which despite all it's problems is still widely used and a good skill to have).

There's a fair amount of vector maths involved here but don't let that intimidate you. I'll try to explain it all well enough that you don't need a maths degree to follow whats going on.

Also, unlike the original book and like the previous project, I'm not going to give you the code snippets as we go. Feel free to take a look at the solutions if you get stuck, but try to solve the tasks yourself as you'll find it much more rewarding. Remember to make use of your resources!

## 1: Images

What does a renderer render? Well... pictures. An image of a scene. So we're going to need some way to output an image in Rust. We're going to take advantage of the excellent crates.io ecosystem here and use a crate called [image](https://github.com/image-rs/image) that does pretty much exactly what it says on the tin: provide tools for working with images. Have a look over the docs over on [docs.rs](https://docs.rs/image/latest/image/) and have a think about how you might go about creating a new image.

We don't need to support any fancy encoding or anything for our raytracer, we just want each pixel to be comprised of 3 bytes: the good old `(r, g, b)`.

### Task 1.1

I'm assuming you've already created a new cargo project and added `image` to your dependencies in `Cargo.toml`. In your main function, write some code to generate an image and save it to disk. The basic steps are something like:

- Create a new image buffer to hold our pixels ([docs](https://docs.rs/image/latest/image/struct.ImageBuffer.html)). 256x256 should be good to start with.
- Iterate over each pixel ([docs](https://docs.rs/image/latest/image/struct.ImageBuffer.html#method.enumerate_pixels_mut))
  - Modify each pixel (try setting each one to a single colour to start with) ([docs](https://docs.rs/image/latest/image/struct.Rgb.html))
- Save the buffer to a file ([docs](https://docs.rs/image/latest/image/struct.ImageBuffer.html#method.save))

### Task 1.2

We're gonna extend the code from above to output something a bit nicer. From here on out, I'm going to talk about RGB values as being floats from 0.0 to 1.0. Converting them to bytes can be done just before you write to the pixel. I'm also going to refer to `i` and `j` as the coordinates of each pixel, where `i` is the offset in columns from the top-left corner, and `j` is the offset in rows (if you're not already using an iterator that does this for you, try find it in the `pixels` documentation).

For each pixel:

- Scale `i` to the range 0.0-1.0 based upon the image's width. This will be your `r` value.
- Scale `j` to the range 0.0-1.0 based upon the image's height. This will be your `g` value.
- Fix `b` at 0.25
- Convert your rgb values to bytes
- Write your pixel to the buffer.

Red will fade from 0 to 1 left to right, and green will fade in from top to bottom. You should get a nice gradient like this:

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

## What next?
