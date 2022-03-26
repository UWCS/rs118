# CHIP-8 Workshop

CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. It was made to allow video games to be more easily programmed for these computers, but CHIP 8 is still used today, due to its simplicity, and consequently on any platform and its teaching of programming Binary numbers. [https://en.wikipedia.org/wiki/CHIP-8](Wikipedia)

In short, its a really basic assembly language-type specification that lets people build neat games, and we're going to build an interpreter for it, applying some of the Rust we've learned so far.

Our emulator is available on [crates.io](https://crates.io/crates/rs118-chip8), so you can install it with `cargo install rs118-chip8` to have a play with it so you can see what your final product should look like. Plenty of roms are available online, we recommend [Space Invaders]() and [Tetris]()

## 0: Getting Started

### Task 0.1: Read the Docs

Before you do anything, have a read of [the CHIP-8 specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) so you have a rough idea of what it is you need to implement and can start thinking about a solution. Think about how you'll represent the different components in Rust.

### Task 0.2: Read More Docs

Create a new cargo project, and open up the `Cargo.toml` file. Our emulator exposes some stuff as a library for you to base your solution around, so add that to your dependencies:

```toml
[dependencies]
rs118-chip8 = "0.1.0"
```

Take a look at the `chip8_base` library that the `rs118-chip8` crate exposes, the documentation for it is available on [docs.rs](). We can see that it exposes two type aliases, `Display` and `Keys`, and also an `Intepreter` trait. The idea is that you create your own interpreter by implementing the trait, and then the `run()` method will run it for you. This works because `run()` is written to accept any type that implements the `Interpreter` trait as an argument.

There's 3 functions our own interpreter will need to provide for a complete trait implementation:

- `step(&mut self, keys: &Keys) -> Option<Display>`
- `speed(&self) -> Duration`
- `buzzer_active(&self) -> bool`

The first one is the main driver function of your interpreter that you'll have to implement, and will execute it one cycle at a time, modifying internal state as necessary. The other two are just helpers to provide the trait with some more info it needs to run your interpreter correctly. Have a think about what these type

### Task 0.3: Git Good

Cargo initialises your new project as a git repo. It does this for a reason, to encourage you to use version control. If you aren't familiar with git, check out our git good resources on our website [LINKS??]. Make a new commit every time you change or try something, so you can keep track of what you've done and roll back if things break. Commit _at least_ when you've done each task.

## 1: The Virtual Machine

The first step we're gonna take is to lay out our program using modules. Refer back to The Book if you need a refresher on how to create modules and structure a Rust program.

### Task 1.1: Modules

Create a new directory next to `main.rs` called `interpreter`, and then add `interpreter/mod.rs`. Add the line `mod interpreter;` to the top of `main.rs` so the `interpreter` module is added to the module tree. This module is where most of our code is going to live. Feel free to create any other modules you wish alongside `mod.rs` too, but don't forget to include them in the module tree.

### Task 1.2: The Interpreter Type

In our new interpreter module we want to create a struct that will represent the state of our CHIP-8 virtual machine.

Create a new struct type, adding any fields that you see fit. Also add an `impl` block for your struct, and create a `new()` associated function to return a default copy of our struct. `new()` can take whatever arguments you see fit to create a new virtual machine.

### Task 1.3: The Interpeter Trait

Before we can use the `run()` function from the `chip8_base` module, we need to tell the compiler that our interpreter is actually an interpreter, by implementing the Interpreter trait on it.

Import the `chip8_base::Interpreter` trait into your interpreter module, and use another `impl` block to implement the trait on your struct. Refer back to The Book if you need a refresher on the exact syntax for this.

### Task 1.4: Shut Up Compiler

The compiler should be screaming at you right about now, because your type is implementing a trait without providing definitions for any of it's methods. Go ahead and add the three required methods to your `impl` block, marking them as [`todo!()`](https://doc.rust-lang.org/std/macro.todo.html) for now.

### Task 1.5: One Step at a Time

So, now you have your own interpreter, the compiler knows its an interpreter, and you can call `step()` on it to step through execution one clock cycle at a time. `main()` is the driver function for our code, so we'll instantiate and run things there.

Head back to `main.rs` and use your `new()` function from the `interpreter` module to create a new virtual machine. Then, call `chip8_base::run()`, passing the type you just instantiated as an argument.

This should all compile now, so `cargo run` it and see what happens!

### Task 1.6: Now I'm Panicking

Well, you left the methods marked as `todo!()`, so it panicked. That was silly. The display window should have still popped up though, but just get rid of that. We can provide some really barebones implementations though that don't do anything. Look at the return type for the three methods and think about what you might want to return.

Make the three `Interpreter` methods return something such that they don't really do anything, but still don't panic.

### Task 1.7: Timing

Have another look at the `speed()` method. To run games properly, the interpreter needs to run at a fix clock speed. The return type of `speed()` is [`Duration`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html), which is Rust's representation of a period of time. You can create `Duration` types with a number of associated functions, take a look at the docs to see which one is appropriate here. Your interpreter type should store some representation of the speed it is currently being run at (A clock speed of 700MHz is generally pretty good for most CHIP-8 games), and be able to return the speed as a `Duration` so that the `run()` function knows exactly how fast to run it.

Make your `speed()` method return a period of time corresponding to the speed of the interpreter. You should not hard-code this, make it a configurable constant or something passed when instantiating a new interpreter, as different games require different speeds.

## 2: Fetch-Decode-Execute

Your interpreter should be happily churning away burning CPU cycles for now, and the display window should work, letting you drag it around, resize it, etc. So, we have a virtual machine that does nothing. That's not very exciting at all, so let's change that. The virtual machine works pretty much the same as a CPU, with a fetch-decode-execute cycle. This entire cycle is one step, and what should be implemented in your step function. The basic of `Interpreter::step()` is:

- Get the next instruction from memory (fetch)
  - Increment the program counter
- Work out what that instruction needs to do (decode)
- Execute the instruction (execute)

### Task 2.1: Fetch

Let's start with fetch. Looking at the spec, we can see that each CHIP-8 opcode is two bytes, composed of four nibbles (each nibble being four bits, or one hex digit). The program counter is 16 bits, and should point to the address of the next instruction to be executed from CHIP-8's 4kB of memory.

Write a function, `fetch`, to return the next opcode to be executed from memory, and increment the program counter. Consider carefully the parameter and return types of your method. Add a call to `fetch` within `step`. If you haven't already got fields for memory and program counter on your struct, now is the time to add them.

### Task 2.2: Fetch, But it Works

So when you run your program now, what should be happening is opcodes are continually fetched from memory, until... it panics? Yes, panics. What's happening is your program is continually fetching until the program counter overflows, which causes a panic when Rust is in debug mode ([see this blog post for a good rundown on overflow in Rust](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/)). To fix this, we need to manually specify a way to make our program counter wrap around back to 0. The program counter is meant to represent a 12-bit address (for CHIP-8's 4096 bytes of memory), so we should wrap back to 0 when it reaches 4096.

Fix your fetch instruction to wrap back to 0 when it reaches the end of the addressable memory. Add in some debug statements (take a look at [the excellent `dbg!()` macro](https://doc.rust-lang.org/std/macro.dbg.html)) to verify that it is fetching continually from memory, returning 0 each time (we haven't loaded anything into memory yet).

### Task 2.3: Decode

So, we can fetch instructions, but what do we do with them? Well, execute them of course. CHIP-8 has 35 instructions with a varying number of operands. We could write a very very long chain of if/else expressions to decode each instruction, or we could use one long `match` expression. Each instruction has four nibbles, some of which are fixed to specify the opcode, some of which are operands of varying length (4, 8 or 12 bit). We can use this to write our `execute()` function (we'll roll decode and execute into one step for simplicity).

Write an `execute` method that uses a `match` expression to decode instructions. There's many ways you could go about this, but we recommend breaking each instruction down into it's four nibbles, then pattern matching on that. For now, make each of your match arms do nothing (return the unit type). Remember that `match` expressions in rust have to cover all possible types, so you can use a wildcard pattern (`_ => ()`) to cover any unimplemented instructions, or instructions that don't exist. **You don't have to do all of the instructions now, just one or two to get the general idea.**. Refer back to [The Book](https://doc.rust-lang.org/book/ch06-02-match.html) (Chapter 18 may also be useful) if you need a refresher on how `match` works.

### Task 2.4: Execute

So we've done fetch and decode, no prizes as to what comes next. Executing an instruction just consists of modifying the internal state of your virtual machine accordingly, so double-check [the specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) at this point to make sure you have all the fields you need on your interpreter struct.

Fill in a few of the arms of your match statement to execute the decoded instructions. Think about how you're going to get the operands out of the instruction when they vary in width. Make your `step()` method call `execute()` so that the interpreter is now doing the fetch-decode-execute cycle. Implement the opcode `0x000` as a NOP (No-Operation) instruction, that simply does nothing.

Congrats! You're successfully emulating a CPU* (*CHIP-8 is not a CPU but its an awful lot like one). Take a moment to think how cool this is, even if it does nothing so far. What should be happening is the interpreter is fetching, decoding and executing `0x0000` instructions continually, which aren't real instructions but we added them because we could.

## 3: The First Few Instructions

In the last section, you implemented one or two random instructions of your choosing, plus our fictitious NOP instruction. Now, we're gonna talk you through implementing a few more, such that you can run a real program. The IBM test ROM does nothing except display the IBM logo, but using only 6 instructions, so those are what we're gonna implement.

### Task 3.1: 00E0 (clear screen)

This instruction should set all of the display's pixels to black. If you don't have some representation of the display in your struct, add one now (look at the [`Display`](https://docs.rs/crates/ADD THE LINK HERE) type if you need a hint).

The `step()` function should return `Some(Display)` when the display is updated, so maybe your execute function wants to do the same, and then the step function should return that? Either way, make sure that executing this instruction causes your updated display state to be returned to `step()`'s caller.

### Task 3.2: 1nnn (jump)

This instruction has one operand, `nnn`, a 12-byte address that should be pushed onto the stack. You can't represent 12-byte values in rust, so pick another type accordingly and include checks/wrapping to make sure that the value remains within 12 bytes. The program counter should simply be set to the value of the operand.

### Task 3.3: 6xnn (set register Vx)

CHIP-8 has 16 general purpose 8-bit registers, numbered `V0` to `VF`. This instruction has two operands, `x`, the register, and `nn`, a byte. The byte should be put in the register. Easy.

### Task 3.4: 7xnn (add to register Vx)

Add the value `nn` to the value in the register `Vx`. This instruction may overflow (causing a panic), so make sure to handle overflow/wrapping correctly.

[**Hint**](https://doc.rust-lang.org/std/primitive.u8.html#method.wrapping_add)

### Task 3.5: Annn (set index register)

The index register is a special 16-byte register, which is generally used to point at locations in memory. This instruction should set the index register to the 12-bit value `nnn`

### Task 3.6: Dxyn (draw)

This is probably the hardest instruction. CHIP-8 has sprites which are 8 bits wide and up to 15 bytes tall. This instruction draws the sprite starting at the address in the index register to the screen at position (`Vx`,`Vy`). Info on how sprites should wrap varies, but generally the X and Y coordinates should be modulo the display size, ie an X-coordinate of 69 should be interpreted as a coordinate of 6, and sprites should not partially wrap. Drawing works by XORing the pixels onto the display, and the `VF` register should also be set to `1` if this causes any pixels to be erased.

- Set `X` to the value in `Vx` modulo 64
- Set `Y` to the value in `Vy` modulo 32
- Zero `VF`
- For each row in the n-byte sprite
  - if y is out of bounds, stop drawing the sprite
  - For each bit in the byte/row
    - If x is out of bounds, stop drawing the row
    - XOR the bit onto the screen
      - Set `VF = 1` if this caused a pixel to be erased.

Check the resources at the bottom (and also google for anything else you can find) for more explanations, as implementations and explanations may vary ever so slightly, but the general idea is aways the same

### Task 3.7: This Tutorial Not Sponsored By IBM

Theoretically, you should be able to run the IBM ROM now ([Link]([)](https://github.com/Joeyh021/rs118-chip8/blob/main/roms/IBM%20Logo.ch8)). But first you need a way to load it into memory. CHIP-8 Programs start at `0x200` in memory, so you need to write a method to load a ROM from disk into memory. [`std::fs::read`](https://doc.rust-lang.org/stable/std/fs/fn.read.html) will load a file from disk and return it as `Vec` of bytes, but how to get it into memory is up to you. You could add it to your `new()` function, or create a separate `load()` function. Make sure you properly handle the `Result` that the `fs::read` returns too, in case you give it a file that doesn't exist.

### Task 3.8: Debugging

Chances are this doesn't work first try (unless you're some next level god tier genius, in which case, congrats). You'll probably need to do some debugging. Making extensive use of the `dbg!()` macro is a good idea, or maybe slow down the speed of your emulator to make it easier to see whats going on step-by-step. Redirecting `stderr` to a file on the command line may come in handy too so you can take a closer look.

If you're using VSCode, debugging Rust is easy. Rust-Analyser adds a little "debug" button above your main function you can press to launch the debugger, allowing you to step through and inspect values one step at a time. If you've never used a debugger in VSCode before, [have a look at this article]
(https://code.visualstudio.com/docs/editor/debugging). [This article includes some information about debugging Rust specifically.](https://dev.to/rogertorres/debugging-rust-with-vs-code-11dj). If you prefer the command line, gdb has support for rust too, through `rust-gdb`.

Writing unit tests to test each instruction in isolation is a good idea too. [Chapter 11 of The Book](https://doc.rust-lang.org/book/ch11-00-testing.html) has some information on writing unit tests in rust, which is incredibly easy. Obviously you should always test your code, but a lot of the opcodes are fairly simple and we don't expect a full suite of unit tests for just a toy project. Writing a few to cover the more complex instructions and check your edge cases is a good idea, as you can then debug the execution of the tests in isolation too.

## 4: The Rest

Well, we've got this far. However, you still have about 30 instructions before you can say you're done. A few test ROMS can be found [here](ADD LINK) for testing all your instructions work. Remember, unit tests are your friend.

Some advice:

- Make sure you implement the `buzzer_active()` function correctly.
- The timer registers will be quite tricky to line the timing up correctly. You can rely on the fact that your `step()` function will be executed once every `t` seconds, where `t` is whatever `Duration` is returned by the `speed()` method.
- Make sure you handle wrapping to 8/12/16 bit boundaries correctly, making use of the standard library's wrapping and saturating add/sub methods.
  - `n & 0xfff` will wrap `n` to a 12 bit boundary
- Some instructions require you set `VF` under certain conditions.
- This is a very specific use case where casting in Rust can be annoying, as CHIP-8 has no strong type system like Rust does. Make sure all your `as` casts are in the right place.
- You don't have to completely re-architect the whole thing to implement the `Fx0A` instruction, trust me. Ask for help with this one if you need.
- You'll need the `rand` rate to generate random numbers
- You'll need to initialise the font in memory at some point. Where/how is best to do this? Font usually starts at 0x50, but can be anywhere in the first 512 bytes.
- Ask for help from a friend or lab tutor, or in [Discord](https://discord.uwcs.co.uk) if you get stuck
- Look at existing implementations if you get really stuck

Not all ROMS you find online will work, as some may be written for Super CHIP-8, an extension of CHIP-8 that adds a few more instructions. Feel free to extend your emulator with these instructions if you want.

## Resources

The CHIP-8 Specification is available at <http://devernay.free.fr/hacks/chip8/C8TECH10.HTM>

A more detailed explanation of each instruction and more of the details of the "hardware" are available at <https://tobiasvl.github.io/blog/write-a-chip-8-emulator/>

A large collection of CHIP-8 stuff is available at <https://chip-8.github.io/links/>
