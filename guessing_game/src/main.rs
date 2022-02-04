// CLI guessing game in rust

// imports
use std::io;

// guessing game

struct GuessingGame {
}

// implements GuessingGame
impl GuessingGame {
    fn new() -> GuessingGame {
        GuessingGame {
        }
    }

    fn guess(&self, n: i32) -> bool {
        n == 42
    }
}


fn main() {
    // Create a new game
    let mut game = GuessingGame::new();
    
    // user input
    let mut input = String::new();
    let mut n = 0;
    loop {
        println!("Guess the number!");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        n = input.trim().parse::<i32>().expect("Please type a number!");
        if game.guess(n) {
            println!("You win!");
            break;
        }
    }


    // Guess a number
    let n = 42;
    let result = game.guess(n);
    println!("{}", result);
}

// to those who think this is bad code - i used github copilot. also first time using the language without a tutoriol