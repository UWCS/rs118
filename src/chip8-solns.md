# CHIP-8 Solutions

These solutions can be seen step-by-step on GitHub [here](https://github.com/ericthelemur/chip9/commits/master).

## Task 1.1

Your directory structure should now look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── interpreter
    │   └── mod.rs
    └── main.rs
```

Which gives a module hierarchy like this:

```
crate root
└── main
    └── interpreter
```

You can add any other modules, for tests or anything else, anywhere you wish.

## Task 1.2

The interpreter/CPU/virtual machine struct should look something like this:

```rust,noplayground
pub struct ChipState {
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],
    // ... there will be more
}
```

Only a few of the fields you need are included here, you'll need to add a few more as you go, and you can represent them however you wish. The corresponding `new()` method should look like this:

```rust,noplayground
impl ChipState {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            program_counter: 0x200,
            display: [[chip8_base::Pixel::default(); 64]; 32],
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
```

Note how both the type and the function are `pub`, so the module above (main, the crate root) can use them. The `program_counter` is initialized to `0x200`, as this is where CHIP-8 programs start.

## Task 1.3 & 1.4

We implement the trait for the type like so

```rust,noplayground
impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        todo!()
    }

    fn speed(&self) -> std::time::Duration {
        todo!()
    }

    fn buzzer_active(&self) -> bool {
        todo!()
    }
}
```

Look at how the methods are capturing `self`. `step()` takes a mutable reference, because it needs to mutate the state of the virtual machine, but it doesn't move, because then we wouldn't be able to do more than one step. The other two take immutable references, because they only need to read state, not modify it.

## Task 1.5

`main()` should look like this:

```rust,noplayground
use interpreter::ChipState;

mod interpreter;

fn main() {
    let vm = ChipState::new();

    chip8_base::run(vm);
}

```

## Task 1.6

The following return values don't do anything, and let the interpreter run without panics:

```rust,noplayground
use std::time::Duration;

...

impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        None
    }

    fn speed(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}
```

## Task 1.7

For a clock rate of 700Hz, you can create a `Duration` using `Duration::from_secs_f64(1_f64/700_f64)`. Don't hardcode this though. The "proper" way to do it is to modify your `new()` method to accept a clock speed, then store the duration in the struct to return when requested.

```rust,noplayground
pub struct ChipState {
    ...
    speed: Duration
}

impl ChipState {
    pub fn new(clock_freq: u32) -> Self {
        Self {
            ...
            speed: Duration::from_secs_f64(1_f64 / clock_freq as f64),
        }
    }
}

...

impl Interpreter for ChipState {
    fn speed(&self) -> Duration {
        self.speed
    }
}
```

## Task 2.1

```rust,noplayground
fn fetch(&mut self) -> u16 {
    let instruction = u16::from_be_bytes([
        self.memory[self.program_counter as usize],
        self.memory[(self.program_counter + 1) as usize],
    ]);
    self.program_counter += 2;
    instruction
}


```

We're capturing by mutable reference, because we need to mutate, but not take ownership.

Look at the [documentation for the `from_be_bytes()`](https://doc.rust-lang.org/stable/std/primitive.u16.html#method.from_be_bytes) method if you don't get what's going on.

There's lots of casting using `as usize` going on, because only a `usize` type can be used to index an array for safety reasons (imagine you used a `u16` type to index an array of 30,000 numbers, it wouldn't make sense semantically). Casting the program counter and other numbers to `usize` is gonna happen a lot, but you can't store them as `usize` types because that wouldn't make sense either, and would also make it much harder to keep track of what a value is meant to represent.

## Task 2.2

The `self.program_counter & 0x0fff;` will wrap the program counter to 12 bits, discarding the upper nibble. Adding some debug calls too:

```rust,noplayground
fn fetch(&mut self) -> u16 {
    dbg!(&self.program_counter);
    let instruction = u16::from_be_bytes([
        self.memory[self.program_counter as usize],
        self.memory[(self.program_counter + 1) as usize],
    ]);
    self.program_counter += 2;
    self.program_counter & 0x0FFF;
    dbg!(&instruction);
    instruction
}
```

We don't have to add any additional info to `dbg!()` because the expression and line number are printed for us.

## Task 2.3

Our main should now look like this:

```rust, noplayground
fn main() {
    env_logger::init();

    let vm = ChipState::new(700);

    chip8_base::run(vm);
}
```

Don't forget to add the crates to `Cargo.toml`. Where you choose to add logs is up to you, but as a rule of thumb, put a `log::debug!()` call everywhere you expect something might go wrong. You can use format strings in log macros too, just like `println!()`.

## Task 2.4 & 2.5

First, we've written a helper method to break the `u16` instruction down into four nibbles (add in `impl ChipState`):

```rust,noplayground
//break a u16 into its nibbles
fn nibbles(n: u16) -> (u8, u8, u8, u8) {
    let n3 = ( n >> 12)          as u8;
    let n2 = ((n >> 8) & 0b1111) as u8;
    let n1 = ((n >> 4) & 0b1111) as u8;
    let n0 = ( n       & 0b1111) as u8;
    (n3, n2, n1, n0)
}
```

We can then match on this. Below shows NOP (`0000`), AND (`8xy2`) and RET (`00EE`) implemented. Here, you could implement almost anything, but this is just an example of the sort of structure you need.

```rust,noplayground
fn execute(&mut self, instruction: u16) {
    match Self::nibbles(instruction) {
        // 0000 NOP: Nothing
        (0x0, 0x0, 0x0, 0x0) => (),
        // 00EE RET: Return from subroutine
        (0x0, 0x0, 0xE, 0xE) => {
            self.program_counter = self.stack[self.stack_pointer as usize];
            self.stack_pointer -= 1;
        },
        // 8xy2 AND Vx, Vy: Set Vx = Vx AND Vy.
        (8, x, y, 2) => self.registers[x as usize] &= self.registers[y as usize],
        _ => panic!("Instruction either doesn't exist or hasn't been implemented yet"),
    }
}
```

Note how we can specify constants in the tuple for the pattern, and also variables to bind to if the pattern matches. How you decode operands wider than one nibble is up to you.

`step()` now looks like this:

```rust,noplayground
fn step(&mut self, keys: &Keys) -> Option<Display> {
    let instr = self.fetch();
    self.execute(instr);
    None
}
```

## Task 3.1

```rust,noplayground
fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
    match Self::nibbles(instruction) {
        // 0000 NOP: Nothing
        (0x0, 0x0, 0x0, 0x0) => (),
        // 00E0 CLS: Clears the display
        (0x0, 0x0, 0xE, 0x0) => {
            self.display = [[chip8_base::Pixel::default(); 64]; 32];
            return Some(self.display);
        }
        _ => panic!("Instruction either doesn't exist or hasn't been implemented yet"),
    };
    None
}
...

impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        let instr = self.fetch();
        self.execute(instr)
    }
    ...
```

Note the return is needed to pass the display back since clear updates the display, also pattern matching on hex to match e.g. `0xE` and stay consistent.

## Task 3.2

```rust,noplayground
fn nnn(instruction: u16) -> u16 {
    instruction & 0x0FFF
}
...

fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
    ...
    // 1nnn JP addr: Jump to location nnn
    (0x1, _, _, _) => self.program_counter = Self::nnn(instruction),
    ...
}
```

Here we use a bitmask to chop off the first bit to get the last 12. This approach disregards the last 3 nibbles in the pattern match, since those variables aren't used, and are taken straight from `instruction` instead. You could also construct `nnn` from those nibbles, though it is more involved.

## Task 3.3

```rust,noplayground
fn kk(instruction: u16) -> u8 {
    (instruction & 0x00FF) as u8
}
...

fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
    ...
    // 6xkk LD Vx, byte: Set Vx = kk.
    (0x6, x, _, _) => self.registers[x as usize] = Self::kk(instruction),
    ...
```

Nearly identical to above, but using `kk` to match the last byte instead of 12 bits.

## Task 3.4

```rust,noplayground
fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
    ...
    // 7xkk ADD Vx, byte: Set Vx = Vx + kk.
    (0x7, x, _, _) => {
        self.registers[x as usize] = self.registers[x as usize].wrapping_add(Self::kk(instruction));
    }
    ...
```

As the hint gave, `wrapping_add` wraps around the overflow as required.

## Task 3.5

Add `index: u16` to the struct and `new`.

```rust,noplayground
fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
    ...
    // Annn LD I, addr: Set I = nnn.
    (0xA,_,_,_) => self.index = Self::nnn(instruction),
    ...
```

## Task 3.6

```rust,noplayground
fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
    ...
    // Dxyn DRW Vx, Vy, nibble: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    (0xD, x, y, n) => {
        let tlx = self.registers[x as usize] % 64;
        let tly = self.registers[y as usize] % 32;
        self.registers[0xF] = 0;
        let ind = self.index as usize;
        let sprite = &self.memory[ind..(ind + n as usize)];

        for (i, row) in sprite.iter().enumerate() {
            let pxy = tly + i as u8;
            if pxy > 31 {
                break;
            }
            
            for j in 0..8 {
                let pxx = tlx + j;
                if pxx > 63 {
                    break;
                }
                let old_px = &mut self.display[pxy as usize][pxx as usize];
                let mask = 2_u8.pow(7 - j as u32);
                let new_u8 = (row & mask) >> (7 - j);
                let new_px: chip8_base::Pixel = new_u8.try_into().unwrap();
                if (new_px & *old_px).into() { // if collision
                    self.registers[0xF] = 1 
                }
                *old_px ^= new_px;
            }
        }
        return Some(self.display)
    },
...
```

This is a translation of the rough pseudocode into Rust. Note how iterating over bits is a bit of a pain. However, iterating over the sprite super is easy: we just grab it as a slice. Remember slices? If not, check [The Book](https://doc.rust-lang.org/book/ch04-03-slices.html)

## Task 3.7

Here is a load function to load a ROM into memory from disk:

```rust,noplayground
pub fn load(mut self, filename: &str) -> std::io::Result<Self> {
    let program = std::fs::read(filename)?;
    self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
    Ok(self)
}
```

Note how this takes ownership, and then returns `std::io::Result<Self>`. We return `Err` if we have some error reading from disk, and the error is returned early to the caller using [`the ? operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html), which we'll cover in more detail next time. Loading a ROM copies the bytes into memory and then moves the PC to the start of the program. Finally, we give ownership back to the caller if everything is okay.

You could also do this capturing `self` by mutable reference, or handle the I/O error here instead of bubbling it up to the caller. All up to you.

## Task 4

You really are on your own here.

Try to ask for help, check your resources, and debug properly first before going straight to the nuclear option of just copying it from my solution, but you can find a full implementation (approximately) following on from this one at [ericthelemur/chip8](https://github.com/ericthelemur/chip8), and one that separates decode and execute at [`rs118-chip8`](https://github.com/uwcs/rs118-chip8).

Note that these solutions are certainly not infallible, so don't rely on it as a source of truth for CHIP-8 implementations!
