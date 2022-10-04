# Tic-Tac-Toe!

We're gonna use everything we learned to put together a neat little game of tic-tac-toe. I'm assuming you all know the rules. If you get stuck on any of the tasks, try to use your resources (The Book, Rust by Example, Google), or ask for someone to help talk you through it, before going straight to the solutions. Remember, the compiler is your friend and will try to tell you where to fix your code when you have an error, and always run `cargo clippy` and `cargo fmt`! (I recommend setting up VS Code to do this for you on save)

## Task 0: `cargo new`

Create a new Cargo project, `cd` into it, and open your editor. Check [The Book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) if you need a reminder on how to do this.

## Task 1: Data Types

We're gonna need some types to represent things within our game. In a game with two players and a board, a `Player` type and a `Board` type both seem sensible.

- There are two players, `X` and `O`
- The board is a 3x3 grid of either an `X` or `O`, or blank.

### Task 1.1

Implement a simple data type to represent players. We could just use strings or numbers to do this, but structuring data properly is key to writing good Rust.

Recall that we can use `struct`s and `enum`s to create our own data types. Which of these could be used to represent our a type with only two different values?

### Task 1.2

Implement a simple data type to represent the state of the game/board.

There are a few ways of approaching this, most of them involving a fixed size array. You'll want to use your player type, but also think about what types can be used to represent something that may or may not be there (`Option`, anyone...?)

### Task 1.3

So we have some players and a game board, but what now? Well it's no good if neither of our players can see the board, so you're going to have to come up with some way of printing the board to the terminal.

Create a new, empty instance of your game board type in main, and write some code to print the empty board. Experiment with manually adding some moves to the board and make sure your code can handle printing `X`s and `O`s properly.

You'll most likely want to iterate through your board array in some way, printing some other characters along with it. You'll need some code to print your `Player` type too (using the [`Display` trait](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html) if you feel fancy), but a simple `match` expression with some `println!()`s will likely do for now.)

**Note:** Rust might not allow you to compare the equality of two custom types so easily. This is also _A Good Thing™_ because the notion of equality is not so simple for all types, so much so that Rust splits it into two traits, `Eq` and `PartialEq`. You will probably want to derive (`#[derive()]`) them for your custom `Player` type to allow you to use `match`, and the `==` and `!=` operators (you may also want `Copy` and `Clone` to make your life easier).

## Task 2: Gaming

You're finally ready to write your game. You'll want a game loop in your main function to do a few things:

- Print the state of the board each turn (T1.3)
- Prompt a player for input (T2.1)
- Add the player's guess to the board (T2.2)
- Check if they've won (T3)
- Move to the next turn (T2.3)

The first bit you've already written, but we need to do the rest too

### Task 2.1

Write some code to prompt for user input in a loop, storing whatever data they enter.

What kind of loop do you want (`for`/`while`/`loop`), and when do you want to break out of it/jump back to the top of it? Consider your control flow carefully here. You'll also need some way to read user input from the terminal. Rust has a [`Stdin` struct](https://doc.rust-lang.org/std/io/fn.stdin.html) with a `read_line()` method in its standard library. [The Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#processing-a-guess) has some good examples of this. You'll need to convert your input from a string into a number too, so check out [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) for some help with that.

### Task 2.2

Now we have player input, we need to use it to update the state of the game. Use the input to add the player's turn to the board, if it is a valid guess. Have a look at `std::io` for input. Numbering squares left-to-right top-to-bottom works well, but if you want to be fancy, how about some chess board style labelling?

What constitutes valid input for a turn? You have 9 squares on your game board, and you can't play where there is already a square in that space. If the guess isn't valid, you'll need to get the player to input a new guess.

### Task 2.3

At the end of each turn, you need to move the game to the next player. Add some code to make sure players take turns properly in your loop, and make sure your game is mostly coherent at this point.

## Task 3: A winner?

Two players should be able to play your game now, taking turns, and specifying only valid moves. But this is no fun if there are no winners.

Add some code to your game loop to see if a move leads to the player winning. If so, print a message to indicate this, and exit the game.

There are multiple cases to consider for a win: 3 rows, 3 columns, and the 2 diagonals. You could hard-code all 8 of these, or save some sanity with some `for` loops. Up to you.
