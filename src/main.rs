use std::{io::{self, BufRead}, fs::File, path::Path};


// Read lines from file
fn get_words<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.expect("Could not read line")).collect()
}

fn get_char_points(character: char) -> u32 {
	match character.to_ascii_lowercase() {
		'a' => 1,
		'b' => 3,
		'c' => 8,
		'd' => 2,
		'e' => 1,
		'f' => 3,
		'g' => 3,
		'h' => 4,
		'i' => 3,
		'j' => 4,
		'k' => 3,
		'l' => 2,
		'm' => 4,
		'n' => 1,
		'o' => 2,
		'p' => 4,
		'r' => 1,
		's' => 2,
		't' => 2,
		'u' => 3,
		'v' => 4,
		'x' => 8,
		'y' => 4,
		'z' => 9,
		'æ' => 4,
		'ø' => 4,
		'å' => 4,
        _ => 0
	}
}

fn get_word_points(word: String) -> u32 {
    if word.len() == 0 {
        return 0
    }
	// sum of points for each character in word
	word
    .chars()
    .map(|char| get_char_points(char))
    .reduce(|a, b| a + b)
    .unwrap()
}

fn get_valid_words(my_letters: String, letters_on_board: String) -> Vec<String> {
    let mut words = get_words("ord.txt");

    let available_letters = my_letters.clone() + letters_on_board.as_str();

    // remove words with too many letters
    words.retain(|word| word.len() <= available_letters.len());

    // remove words that does not contain all letters on board
    words.retain(|word| {
        let chars = word.to_lowercase();
        for letter in letters_on_board.chars() {
            if !chars.contains(letter) {
                return false;
            }
        }
        return true;
    });

    // remove words that cannot be made with available_letters
    words.retain(|word| {
        let mut letters = available_letters.clone();
        for letter in word.to_lowercase().chars() {
            if !letters.contains(letter) {
                return false;
            }
            let index = letters.find(letter).unwrap();
            letters.remove(index);
        }
        return true;
    });

    // remove words that do not use a letter from our hand
    words.retain(|word| {
        for letter in word.to_lowercase().chars() {
            if my_letters.contains(letter) {
                return true;
            }
        }
        return false;
    });

    words
}

fn main() {
    let my_letters = "yosåfye".to_owned();
    let letters_on_board = "s".to_owned();
    let mut valid_words = get_valid_words(my_letters, letters_on_board);
    // sort by points
    valid_words.sort_by(|a, b| get_word_points(b.to_string()).cmp(&get_word_points(a.to_string())));
    valid_words.reverse(); // reverse to see highest points easier

    for word in valid_words {
        println!("{}: {}", word, get_word_points(word.to_string()));
    }
}
