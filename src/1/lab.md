# Lab Sheet

We're gonna use everything we learned to put together a neat little game of tic-tac-toe. I'm assuming you all know the rules.

## Task 0: Create a new project

**Task 0.0**: Create a new Cargo project, `cd` into it, and open your editor. Check [The Book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) for info on how to do this.

## Task 1: Data Types

We're gonna need some types to represent things within our game. In a game with two players and a board, a `Player` type and a `Board` type both seem sensible.

- There are two players, either `X` or `O`
- The board is a 3x3 grid of either an `X` or `O`, or blank.

**Task 1.1**: Implement a simple data type to represent players. We could just use a string or number to do this, but structuring data properly is key to writing good rust.

<details>
    <summary>Hint</summary>
    <p>
    What type can be used to represent a set of discrete values, and can take on one of those values at once?
    </p>
</details>

**Task 1.2**: Implement a simple data type to represent the state of the game/board.

<details>
    <summary>Hint</summary>
    <p>
    There's a few ways of approaching this, but all of them will involve a fixed size array. You'll want to use your player type, but also think about what types can be used to represent something that may or may not be there.
    </p>
</details>

**Task 1.3**: Write some code in `main()` to print your game board.

<details>
    <summary>Hint</summary>
    <p>
    You'll most likely want to iterate through your board array in some way, printing some other characters along with it. Have a look at the Display trait too, if you want some help printing your player type.
    </p>
</details>

## Task 2: The Game Loop

You're finally ready to write your game. You'll want a main game loop in your main function to do a few things:

- Print the state of the board each turn
- Prompt a player for input
- Add the player's guess to the board
- Check if they've won
- Move to the next turn

The first bit you've already written, but we need to do the rest too

**Task 2.1**: Write some code to prompt for user input in a loop

<details>
    <summary>Hint</summary>
    <p>
    What kind of loop do you want here, and when do you want to break out of it/jump back to the top of it? Consider your control flow carefully here.
    </p>
</details>

<details>
    <summary>Hint</summary>
    <p>
    You'll need some way to read user input from stdin. Check The Book, or Rust by Example, to see if there are any examples for this.
    </p>
</details>

Now we have player input, we need to use it to update the state of the game.

**Task 2.2** is to use the input to add the player's guess to the board, if it is valid, and then move to the next turn.

<details>
    <summary>Hint</summary>
    <p>
    What constitutes a valid guess? Consider what input validation you'll need to do for this.
    </p>
</details>

## Task 3: A winner?

Two players should hopefully be able to play your game now, taking turns, and specifying only valid moves. But this is no fun if there's no winners, because that's what it's all about after all.

**Task 3**: Add some code to your game loop to see if a move leads to the player winning. If so, print a message to indicate this, and exit the game.

<details>
    <summary>Hint</summary>
    <p>
    There are multiple cases to consider for a win: 3 rows, 3 columns, and the 2 diagonals. You could hard-code all these, or you could get fancy with some for loops. Up to you.
    </p>
</details>

<details>
    <summary>Hint</summary>
    <p>
    Rust might not allow you to compare the equality of two types so easily. This is also A Good Thing (tm) because the notion of equality is not so simple for all types, so much so that Rust splits it into two traits, Eq and PartialEq. You might want to derive them for your custom player type to help with this. Traits, and deriving them, will be covered in more detail next time.
    </p>
</details>
