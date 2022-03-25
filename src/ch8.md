# CHIP-8 Workshop

CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. It was made to allow video games to be more easily programmed for these computers, but CHIP 8 is still used today, due to its simplicity, and consequently on any platform and its teaching of programming Binary numbers. [https://en.wikipedia.org/wiki/CHIP-8](Wikipedia)

In short, its a really basic assembly language-type specification that lets people build neat games, and we're going to build an interpreter for it, applying some of the Rust we've learned so far.

Our emulator is available on [crates.io](https://crates.io/crates/rs118-chip8), so you can install it with `cargo install rs118-chip8` to have a play with it so you can see what your final product should look like. Plenty of roms are available online, we recommend

## Resources

The CHIP-8 Specification is available at <http://devernay.free.fr/hacks/chip8/C8TECH10.HTM>

A more detailed explanation of each instruction and more of the details of the "hardware" are available at <https://tobiasvl.github.io/blog/write-a-chip-8-emulator/>

A large collection of CHIP-8 stuff is available at <https://chip-8.github.io/links/>

## Getting Started

Create a new cargo project, and open up the `Cargo.toml` file. Our emulator exposes some stuff as a library for you to base your solution around, so add that to your dependencies:

```toml
[dependencies]
chip8_base = "0.1.0"
```

## The "CPU"

(CHIP-8 was never a physical piece of hardware, but it did emulate a kind of CPU-type thing, hence "CPU".)

Create a new file next to `main.rs` called `cpu.rs`. This will be where our CPU lives.

cpu struct and methods, fill in blanks

## Fetch-Decode-Execute

how should step work

## The First Few Instructions

what do we need for IBM
more handholdy

## The Rest

youre on your own

general tips:
wrapping to u8 and u12/u16 boundaries
might need rand crate for rand
