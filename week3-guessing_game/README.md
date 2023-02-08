# Guessing game using Rust
Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message with the number of guessing time and exit.

# Execution

- To run, `cargo run`
- Enter a randome integer between 1 and 100
    - e.g. Please input your guess.
           `67`
- The expected output will be
```
Guess the number!
Please input your guess.
34
You guessed: 34
Too small!
Please input your guess.
67
You guessed: 67
Too small!
Please input your guess.
78
You guessed: 78
Too big!
Please input your guess.
76
You guessed: 76
You win!
You guessed 4 times
```
![rule](https://github.com/nogibjj/Chloe-rust-mini-projects/blob/main/week3-guessing_game/src/Guessing-Game-Pic-full.png)
