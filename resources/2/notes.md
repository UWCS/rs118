# Lesson 2 - borrowck 1-2

Gonna cover ownership. Shit will start to get confusing, but eventually will click and you'll see why all your C code doesn't work.

## Ownership ([4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html))

Rust does not have a garbage collector like Java/Python/C#/Go. Instead, it allows you to manually manage memory, but within a set of constraints enforced by the compiler:

- Every value in rust has a variable that is it's **owner**
- There can only be **one owner at a time**
- When the owning variable goes out of scope, the value will be **dropped**

If you're not familiar with the stack and the heap, read [this](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap) for a quick overview.

In terms of scope in rust, this can broadly be interpreted in the way that you probably think. A variable is valid from the point at which it is delcared, until the end of the current scope

```rust
{ //s not valid here, not declared yet
    let s = "string"; // s is valid from here down

}// this scope ends here, s is no longer valid
```

I'm going to use the `String` type as an example, which hopefully you have already seen. String is more complex than types we've seen or written so far, because it represents a mutable string that may vary in length. This is as oppose to **string literals**, like `s` in the example above.

The value of string literals are hardcoded into the program, meaning if you were to compile the below program, you'd find the actual values of the characters in the executable (you can use the `strings` command, probably combined with `grep`, to verify this).

```rust
fn main(){
    println!("Hello from your binary file!");
}
```

`String`, instead, stores the string data on the heap, which allows us to work with text that is unknown at compile time. `String`s can be mutated:

```rust
{
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`
} // s goes out of scope here
```

Since data on the heap is being used, we need to:

- Allocate that memory when we create the string
- Modify the buffer as we grow the string
- Free the memory when we're done with it

Rust is a big fan of _zero-cost abstraction_, meaning that lots of these details that would ordinarily have to be done manually in C/C++, are abstracted away behind `String`'s implementation details. The zero-cost bit mean's its just as fast as doing it manually, so we get all this simplicity for free.

- Memory is allocated by the call to `String::new()` or `String::from()`
- The buffer is modified behind the scenes when we modify the string's size using `String::push_str()`
- The memory is **automatically freed when the owning variable goes out of scope**

This last bit is really key. Using the example above, we can see that `s` goes out of scope at the closing brace. `s` is the variable that _owns_ the string, so the as the owner goes out of scope, the memory is freed. This is done using a special function called [`drop()`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), which rust calls for us and is implemented automatically for most types.

(This concept will be familiar to you if you know about the concept of Resource Acquisition Is Initialization (RAII) in C++)

### Moves

What do you think this code does ?

```rust
let x = 5;
let y = x;
```

How about this?

```rust
let s1 = String::from("hello");
let s2 = s1;
```

The first example binds the value `5` to the variable `x`, and then the second line makes a **copy** of the variable `x` and binds it to `y`.

The second example does not do this.

`String` is a Struct made up of 3 parts:

- A number representing it's length
- A number representing the capacity of it's internal buffer
- A **pointer** to the buffer where the data is.

(For those of you not familiar with pointers, a pointer is just a number representing an address in memory. It is a variable that says something like "hello, the data you're looking for isn't here, but it can be found at this memory address!". The idea is that you then go and look at that memory address on the heap to go get your data.)

Our string looks a little something like this (the left values are on the stack, the heap to the right):

![](https://doc.rust-lang.org/book/img/trpl04-01.svg)

So to copy our data, we could copy both the heap and stack data, giving us a view in memory like this:

![](https://doc.rust-lang.org/book/img/trpl04-03.svg)

This is also not what happens (imagine if your string was very very long, this would be a very expensive operation).

So what we could do is copy only the data on the stack (the three numbers), giving us a view of the two variables that looks like this:

![](https://doc.rust-lang.org/book/img/trpl04-02.svg)

This is not what happens. Remember that when variables go out of scope, `drop()` is called and their memory is freed. If this were the case, the pointer would be freed twice, and this is a big error that can cause the heap to be corrupted.

What _actually_ happens, is that when you create a copy of the `String`, it's values are **moved**.

![](https://doc.rust-lang.org/book/img/trpl04-04.svg)

A [**shallow copy**](https://stackoverflow.com/questions/184710/what-is-the-difference-between-a-deep-copy-and-a-shallow-copy) is made, copying only the stack data, and not the heap data. To avoid the issue of multiple pointers pointing to the same data **the old variable is invalidated**. Try to run this:

```rust
  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}, world!", s1);
```

You can't, because `s1` was invalidated when you moved it into `s2`, and `s2` became the new owner.

### `Copy` and `Clone`

Of course, this wasn't the case for our first example with the integers:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Both variables are still valid. This is because, for certain types which are

- Small/cheap to copy
- Have a known size at compile time
- Live on the stack

Rust disregards the whole move semantics thing and just makes copies all over the place, because it is basically free to do so. This is done via the `Copy` trait, a trait that tells the compiler that the type is basically free to copy. If a type is `Copy`, then you don't have to worry about move semantics. The following types are `Copy` by default:

- All integral types, such as `u32`
- `bool` (`true`/`false`)
- Floats `f64` and `f32`
- `char`s
- Tuples, but only if they contain types that are also `Copy`.
  - `(u32,f64)` is `Copy` but `(char, String)` is not

There are other ways to make copies of data too. The `Clone` trait can be implemented on any type, which gives it a `clone()` method. `clone()` may be called explicitly to make a copy of your data. The following code clones the data in `s1` into `s2`, meaning both variables own separate copies of the data.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

- `Clone` is used to allow types to be explicitly copied when needed, and indicates that this is an expensive operation which may take time
- `Copy` is used to tell the compiler that types may be copied for free, and that move semantics may be disregarded for this type

To learn how to add these traits to your types, see [Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).

### Ownership and Functions

The semantics for passing values to functions follow the same rules, it will either move or copy, just like assigning to a variable.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

We can't use `s` after `take_ownership()` is called, as it is moved into the function, then the function scope ends and it is dropped.

Functions return values, which gives ownership too. Assigning the result of a function call to a variable makes that variable the new owner:

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

## References and Borrowing ([4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html))

## Slices ([4.3](https://doc.rust-lang.org/book/ch04-03-slices.html))

## `Vec`s ([8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html))

## `String` and `&str` ([8.2](https://doc.rust-lang.org/book/ch08-02-strings.html))

## Error Handling ([9](https://doc.rust-lang.org/book/ch09-00-error-handling.html))

## The Structure of a Rust Program ([7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html))

```

```

```

```

```

```

```

```
