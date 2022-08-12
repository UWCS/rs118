# CHIP-8 Solutions

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
pub struct VM {
    memory: [u8; 4096],
    pc: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack: Vec<u16>
}
```

Only a few of the fields you need are included here, you'll need to add a few more as you go, and you can represent them however you wish. The corresponding `new()` method should look like this:

```rust,noplayground
impl VM {
    pub fn new() -> Self {
        VM {
            memory: [0; 4096],
            pc: 0,
            registers: [0; 16],
            display: [[0; 64]; 32]
            stack: Vec::new(),
        }
    }
}
```

Note how both the type and the function are `pub`, so the module above (main, the crate root) can use them.

## Task 1.3 & 1.4

We implement the trait for the type like so

```rust,noplayground
use chip8_base::Interpreter;

impl Interpreter for VM {
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        todo!()
    }

    fn speed(&self) -> Duration {
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
mod interpreter;
use chip8_base::run;
use interpreter::VM;

fn main() {
    let vm = VM::new();
    run(vm);
}
```

## Task 1.6

The following return values don't do anything, and let the interpreter run without panics:

```rust,noplayground
impl Interpreter for VM {
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        None
    }

    fn speed(&self) -> Duration {
        //don't make this zero, run() will panic because it can't run at infinite speed
        Duration::from_secs(1)
    }

    fn buzzer_active(&self) -> bool {
        //if this was true your computer would beep constantly
        false
    }
}
```

## Task 1.7

For a clock rate of 700Hz, you can create a `Duration` using `Duration::from_secs_f64(1_f64/700_f64)`. Don't hardcode this though. The "proper" way to do it is to modify your `new()` method to accept a clock speed, then store the duration in the struct to return when requested.

```rust,noplayground
impl VM {
    pub fn new(clock_speed: u32) -> Self {
        Self {
            //... rest of the fields
            speed: Duration::from_secs_f64(1_f64/ clock_speed as f64),
        }
    }
}

impl Interpreter for VM {
    fn speed(&self) -> Duration {
        self.speed
    }
}
```

## Task 2.1

```rust,noplayground
fn fetch(&mut self) -> u16 {
    let instruction = u16::from_be_bytes([
        self.memory[self.pc as usize],
        self.memory[(self.pc + 1) as usize],
    ]);
    self.pc += 2;
    instruction
}
```

We're capturing by mutable reference, because we need to mutate, but not take ownership.

Look at the [documentation for the `from_be_bytes()`](https://doc.rust-lang.org/stable/std/primitive.u16.html#method.from_be_bytes) method if you don't get what's going on.

There's lots of casting using `as usize` going on, because only a `usize` type can be used to index an array for safety reasons (imagine you used a `u16` type to index an array of 30,000 numbers, it wouldn't make sense semantically). Casting the program counter and other numbers to `usize` is gonna happen a lot, but you can't store them as `usize` types because that wouldn't make sense either, and would also make it much harder to keep track of what a value is meant to represent.

## Task 2.2

The `self.pc & 0x0fff;` will wrap the program counter to 12 bits, discarding the upper nibble. Adding some debug calls too:

```rust,noplayground
fn fetch(&mut self) -> u16 {
    dbg!(&self.pc);
    let instruction = u16::from_be_bytes([
        self.memory[self.pc as usize],
        self.memory[(self.pc + 1) as usize],
    ]);
    self.pc +=2;
    self.pc & 0xfff;
    dbg!(&instruction);
    instruction
}
```

We don't have to add any additional info to `dbg!()` because the expression and line number are printed for us.

## Task 2.3 & 2.4

First, we've written a helper method to break the `u16` instruction down into four nibbles:

```rust,noplayground
//break a u16 into its nibbles
fn nibbles(n: u16) -> (u8, u8, u8, u8) {
    let n3 = (n >> 12) as u8;
    let n2 = ((n >> 8) & 0b1111) as u8;
    let n1 = ((n >> 4) & 0b1111) as u8;
    let n0 = (n & 0b1111) as u8;
    (n3, n2, n1, n0)
}
```

We can then match on this. Below shows NOP (`0000`), AND (`8xy2`) and RET (`00EE`) implemented.

```rust,noplayground
fn execute(&mut self, instruction: u16) {
    match nibbles(instruction) {
        // 0000 NOP: Nothing
        (0x0, 0x0, 0x0, 0x0) => ()
        // 00EE RET: Return from subroutine
        (0x0, 0x0, 0xE, 0xE) => self.pc = self.stack.pop().unwrap_or(0),
        // 8xy2 AND Vx, Vy: Set Vx = Vx AND Vy.
        (8,x,y,2) => self.registers[x as usize] &= self.registers[y as usize],
        _ => panic!("Instruction either doesn't exist or hasn't been implemented yet"),
    }
}
```

Note how we can specify constants in the tuple for the pattern, and also variables to bind to if the pattern matches. How you decode operands wider than one nibble is up to you.

`step()` now looks like this:

```rust,noplayground
fn step(&mut self, keys: &Keys) -> Option<Display> {
    let instruction = self.fetch();
    self.execute(instruction);
    None
}
```

## Task 3.1 - 3.6

The execute function has been extended to implement the 6 instructions required:

```rust,noplayground
fn execute(&mut self, instruction: u16) -> {
    //helpers that get variable length operands with some bit masking/casting
    let nnn: u16 = twelvebit(opcode);
    let kk: u8 = eightbit(opcode);

    match nibbles(instruction) {
        (0,0,0,0) => (), //NOP
        (0,0,0xE,0) => return Some([[0; 64]; 32]) //CLS
        (1,_,_,_) => self.pc = addr,//JMP
        (6,x,_,_) => self.registers[x as usize] = kk, //SETR
        (7,x,_,_) => self.registers[x as usize] = self.registers[x as usize].wrapping_add(kk), //ADDR
        (0xA,_,_,_) => self.index = nnn, //SETI
        (0xD,x,y,n) => => { //DRAW
        let range = (self.index as usize)..((self.index + n as u16) as usize);
        let sprite = &self.memory[range];
        let x = self.registers[rx as usize] % 64;
        let y = self.registers[ry as usize] % 32;
        self.registers[0xf] = 0;
        for (i, row) in sprite.iter().enumerate() {
            if y + i as u8 > 31 {
                break;
            }
            for (j, sprite_px) in (0..8).zip(PixIterator::new(row)) {
                if x + j as u8 > 63 {
                    break;
                }
                let display_px = &mut self.display[(y as usize + i)][(x as usize + j)];
                //set vf on collide
                if *display_px == 1 && sprite_px == 1 {
                    self.registers[0xf] = 1;
                }
                //xor onto display
                *display_px ^= sprite_px;
            }
        }
        return Some(self.display);
        }
        _ => panic!("Instruction either doesn't exist or hasn't been implemented yet"),
    };
    None
}
```

A few things going on here:

- The two helper functions at the top grab the variable length operands out of the opcode if we need.
  - You can do this however you want, don't just blindly follow our solution.
- Where the operands are wider than one nibble, we bind them with wildcards instead of variables because we don't need the single nibble, we need wider than that, which is handled by the two helper functions at the top.
- The two instructions that modify display state return early, and then a default return of `None` is added at the bottom
  - You'll want to modify `step` to return the display updates too.
- `Dxyn` is just a translation of the rough pseudocode into Rust. Note how iterating over bits is a pain, however, but iterating over the sprite is easy: we just grab it as a slice. Remember slices? If not, check [The Book](https://doc.rust-lang.org/book/ch04-03-slices.html)
  - Bounds checks are included on each iteration, but if the entire sprite is off-screen then it wraps.

## Task 3.7

Here is a load function to load a ROM into memory from disk:

```rust,noplayground
pub fn load(mut self, filename: &str) -> std::io::Result<Self> {
    let program = std::fs::read(filename)?;
    self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
    self.pc = 0x200;
    Ok(self)
}
```

Note how this takes ownership, and then returns `std::io::Result<Self>`. We return `Err` if we have some error reading from disk, and the error is returned early to the caller using [`the ? operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html), which we'll cover in more detail next time. Loading a ROM copies the bytes into memory and then moves the PC to the start of the program. Finally, we give ownership back to the caller if everything is okay.

You could also do this capturing `self` by mutable reference, or handle the I/O error here instead of bubbling it up to the caller. All up to you.

## Task 4

You really are on your own here.

Try to ask for help, check your resources, and debug properly first before going straight to the nuclear option of just copying it from my solution, but you can find the `rs118-chip8` implementation [on Github](https://github.com/uwcs/rs118-chip8)

Note that my solution is also far from perfect, so don't rely on it as a source of truth for CHIP-8 implementations!
