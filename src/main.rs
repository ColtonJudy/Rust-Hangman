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
    println!("Guess a letter: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("\n");

    if input.trim().len() != 1 {
        eprintln!("Please enter a single letter");
        ' '
    } else {
        input.trim().parse().expect("Failed to get character")
    }
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

fn display_hangman(wrong_guesses: i32) {
    let hangman = match wrong_guesses {
        1 => {
            r#"
  _______
  |     |
  |     
  |     
  |     
  |     
  |______"#
        }
        2 => {
            r#"
  _______
  |     |
  |     O
  |     
  |     
  |     
  |______"#
        }
        3 => {
            r#"
  _______
  |     |
  |     O
  |     |
  |     
  |     
  |______"#
        }
        4 => {
            r#"
  _______
  |     |
  |     O
  |    /|
  |     
  |     
  |______"#
        }
        5 => {
            r#"
  _______
  |     |
  |     O
  |    /|\
  |     
  |     
  |______"#
        }
        6 => {
            r#"
  _______
  |     |
  |     O
  |    /|\
  |    / 
  |     
  |______"#
        }
        7 => {
            r#"
  _______
  |     |
  |     O
  |    /|\
  |    / \
  |     
  |______"#
        }
        _ => "",
    };

    println!("{}\n", hangman);
}

fn main() {
    println!("Welcome to Hangman\n");

    let mut exit_flag = false;
    let word_to_guess = get_word();
    let mut display_word: String = make_display_word(&word_to_guess);

    println!("{display_word}\n");

    let mut wrong_guesses = 0;

    while exit_flag == false {
        let guess = handle_guess();

        if guess != ' ' {
            if modify_display_word(&mut display_word, &word_to_guess, guess) {
                println!("You guessed correct\n");
                println!("{}\n", display_word);
            } else {
                println!("You guessed incorrect\n");
                wrong_guesses += 1;
                display_hangman(wrong_guesses);
                println!("{}\n", display_word);
            }

            if wrong_guesses > 6 {
                println!("You lose!");
                println!("The word was {}", word_to_guess);
                exit_flag = true;
            } else if display_word == word_to_guess {
                println!("You win!");
                exit_flag = true;
            }
        }
    }
}
