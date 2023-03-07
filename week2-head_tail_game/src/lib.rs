

use rand::seq::SliceRandom;

pub fn game(choice: String) {
    let choices = vec!["head", "tail"];
    let computer_choice = choices.choose(&mut rand::thread_rng()).unwrap();
    let result = match (choice.as_str(), *computer_choice) {
        ("head", "head") => "You win!",
        ("head", "tail") => "You lose!",
        ("tail", "head") => "You lose!",
        ("tail", "tail") => "You win!",
        _ => "You didn't choose head or tail.",
    };
    println!(
        "You chose: {}\nComputer chose: {}\nResult: {}\n",
        choice, computer_choice, result
    );
}