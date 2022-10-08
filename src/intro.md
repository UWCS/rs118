# Introduction

Hello, and welcome to RS118! This is a short series of lectures and projects designed to introduce beginners to programming in Rust. There is a basic level of programming knowledge assumed throughout.

This tutorial will heavily lean on and is adapted from [_The Rust Programming Language_](https://doc.rust-lang.org/book/), more commonly known as **The Book**. Book chapters and links are referenced throughout, and it is recommended you read the entire chapter, as these notes are just here as a brief summary. Other resources:

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - If you're looking for an example for something to explain how to do it or how it works, look here
- [The Reference](https://doc.rust-lang.org/stable/reference/) - This is a complete\* reference for the Rust language. It is quite detailed and technical, but very thorough.
- [Rustlings](https://github.com/rust-lang/rustlings) - Lots of little exercises and examples to demonstrate specific concepts

The code snippets in this book can be run using the play button in the top right corner:

```rust
println!("Hello, Ferris!");
```

Some can also be edited, to allow you to play around with the concepts being presented. Try to fix the error in the code below:

```rust, editable
fn main() {
    let message: &str = "Edit me!"
    println!("Your message says: {message}");
}
```

I encourage you to play around with the snippets to help get a better understanding of how the compiler works.

The source for this book is available [on GitHub](https://github.com/uwcs/rs118), and contributions/corrections/suggestions/additions are welcome.

RS118 is kindly supported by an event grant from [The Rust Foundation](https://foundation.rust-lang.org/). They do a lot of really important stuff for the language, so go show them some love.

This book and all the code associated with RS118 is distributed under the terms of the MIT licence, Copyright 2022 Joey Harrison & The University of Warwick Computing Society. If you use anything from this book or the associated GitHub repos, please give credit.
