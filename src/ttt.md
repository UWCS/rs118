# Tic-Tac-Toe!

We're gonna use everything we learned to put together a neat little game of tic-tac-toe. I'm assuming you all know the rules. If you get stuck on any of the tasks, try to use your resources (The Book, Rust by Example, Google), or ask for someone to help talk you through it, before going straight to the solutions.

## Task 0: Create a new project

**Task 0.0**: Create a new Cargo project, `cd` into it, and open your editor. Check [The Book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) if you need a reminder on how to do this.

## Task 1: Data Types

We're gonna need some types to represent things within our game. In a game with two players and a board, a `Player` type and a `Board` type both seem sensible.

- There are two players, `X` and `O`
- The board is a 3x3 grid of either an `X` or `O`, or blank.

**Task 1.1**: Implement a simple data type to represent players. We could just use strings or numbers to do this, but structuring data properly is key to writing good rust.

Recall that we can use `struct`s and `enum`s to create our own data types. Which of these could be used to represent our a type with only two different values?

**Task 1.2**: Implement a simple data type to represent the state of the game/board.

There's a few ways of approaching this, most of them involving a fixed size array. You'll want to use your player type, but also think about what types can be used to represent something that may or may not be there (Option, anyone...?)

So we have some players and a game board, but what now? Well its no good if neither of our players can see the board, so you're going to have to come up with some way of printing the board to the terminal.

**Task 1.3**: Create a new, empty instance of your game board type in main, and write some code to print the empty board. Experiment with manually adding some moves to the board and make sure your code can handle printing Xs and Os properly.

You'll most likely want to iterate through your board array in some way, printing some other characters along with it. You'll need some code to print your `Player` type too (if you want to get fancy with this, check out the [`Display` trait](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html), but a simple `match` expression will likely do for now.)

## Task 2: Gaming

You're finally ready to write your game. You'll want a game loop in your main function to do a few things:

- Print the state of the board each turn
- Prompt a player for input
- Add the player's guess to the board
- Check if they've won
- Move to the next turn

The first bit you've already written, but we need to do the rest too

**Task 2.1**: Write some code to prompt for user input in a loop, storing whatever data they enter.

What kind of loop do you want (`for`/`while`/`loop`), and when do you want to break out of it/jump back to the top of it? Consider your control flow carefully here. You'll also need some way to read user input from the terminal. Rust has a [`Stdin` struct](https://doc.rust-lang.org/std/io/fn.stdin.html) with a `read_line()` method in it's standard library. The Book and Rust by Example have some good examples of this.

Now we have player input, we need to use it to update the state of the game.

**Task 2.2**: use the input to add the player's turn to the board, if it is a valid guess.

What constitutes valid input for a turn? You have 9 squares on your game board, and you can't play where there is already a square in that space. If the guess isn't valid, you'll need to get the player to input a new guess.

**Task 2.3**: At the end of each turn, you need to move the game to the next player. Add some code to make sure players take turns properly in your loop, and make sure you game is mostly coherent at this point.

## Task 3: A winner?

Two players should hopefully be able to play your game now, taking turns, and specifying only valid moves. But this is no fun if there's no winners, because that's what it's all about after all.

**Task 3**: Add some code to your game loop to see if a move leads to the player winning. If so, print a message to indicate this, and exit the game.

There are multiple cases to consider for a win: 3 rows, 3 columns, and the 2 diagonals. You could hard-code all 8 of these, or you could get fancy with some for loops. Up to you.

**Note:** Rust might not allow you to compare the equality of two custom types so easily. This is also A Good Thing (tm) because the notion of equality is not so simple for all types, so much so that Rust splits it into two traits, `Eq` and `PartialEq`. You might want to derive them for your custom `Player` type to help with this. Traits, and deriving them, will be covered in more detail next time, but [a good overview](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) is given in The Book.
