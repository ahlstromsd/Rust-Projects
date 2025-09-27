use std::io;
use std::collections::HashSet;

fn main() {
    println!("Welcome to Hangman!");

    let word = String::from("rust"); // secret word
    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut attempts_left = 6;

    while attempts_left > 0 {
        display_word(&word, &guessed_letters);

        println!("Enter a letter: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess_char = match guess.trim().chars().next() {
            Some(c) => c,
            None => {
                println!("Invalid input!");
                continue;
            }
        };

        if guessed_letters.contains(&guess_char) {
            println!("You already guessed '{}'", guess_char);
            continue;
        }

        guessed_letters.insert(guess_char);

        if word.contains(guess_char) {
            println!("Good guess!");
        } else {
            attempts_left -= 1;
            println!("Wrong! Attempts left: {}", attempts_left);
        }

        if word.chars().all(|c| guessed_letters.contains(&c)) {
            println!("You win! The word was '{}'", word);
            return;
        }
    }

    println!("Game over! The word was '{}'", word);
}

fn display_word(word: &str, guessed: &HashSet<char>) {
    let display: String = word
        .chars()
        .map(|c| if guessed.contains(&c) { c } else { '_' })
        .collect();
    println!("Word: {}", display);
}
