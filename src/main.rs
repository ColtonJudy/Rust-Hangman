use rand::Rng;

fn get_word() -> String {
    let words: Vec<String> = std::fs::read_to_string("src/words.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=499);

    let word = words[random_number as usize].clone();
    word
}

fn make_display_word(word_to_guess: &String) -> String {
    "_".repeat(word_to_guess.len())
}

fn handle_guess() -> char {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please enter a character")
}

fn modify_display_word(display_word: &mut String, word_to_guess: &String, guess: char) -> bool {
    let mut guess_is_correct = false;

    for (i, c) in word_to_guess.chars().enumerate() {
        if c == guess && display_word.chars().nth(i).unwrap() != guess {
            display_word.replace_range(i..=i, &guess.to_string());
            guess_is_correct = true;
        }
    }

    guess_is_correct
}

fn display_hangman(wrong_guesses: i32, exit_flag: &bool) {
  
}

fn main() {
    println!("Welcome to Hangman");
    let mut exit_flag = false;
    let word_to_guess = get_word();
    let mut display_word: String = make_display_word(&word_to_guess);
    println!("{display_word}");
    let mut wrong_guesses = 0;

    while exit_flag == false {
        let guess = handle_guess();

        if modify_display_word(&mut display_word, &word_to_guess, guess) {
            println!("You guessed correct\n\n");
            println!("{}", display_word);
        } else {
            println!("You guessed incorrect\n\n");
            wrong_guesses += 1;
            display_hangman(wrong_guesses, &exit_flag);
            println!("{}", display_word);
        }

        if wrong_guesses > 7 {
            println!("You lose!");
            println!("The word was {}", word_to_guess);
            exit_flag = true;
        } else if display_word == word_to_guess {
            println!("You win!");
            exit_flag = true;
        }
    }
}
