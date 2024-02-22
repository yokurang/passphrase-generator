/*
    A simple passphrase generator using Markov chains.
*/

use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn clean_word(word: &str) -> String {
    /*
    A function to normalize words by
        1. Removing non-alphabetic characters
        2. Converting to lowercase
    */
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

fn create_transition_matrix(words: Vec<String>) -> HashMap<(String, String), Vec<String>> {
    /*
        A function to create a transition matrix from a list of words.
        Heuristic states that the next word is dependent on the previous two words.
    */

    let mut transition_matrix = HashMap::new();
    for window in words.windows(3) {
        if let [w0, w1, w2] = &window {
            let entry = transition_matrix
                .entry((w0.clone(), w1.clone()))
                .or_insert_with(Vec::new);
            entry.push(w2.clone());
        }
    }
    transition_matrix
}

fn markov_chain(
    transition_matrix: &HashMap<(String, String), Vec<String>>,
    length: usize,
    w0: String,
    w1: String,
    w2: String,
) -> Vec<String> {
    /*
        A function to apply a Markov chain to generate the next word in a sequence.
    */
    let mut rng = rand::thread_rng();
    let mut result = vec![w2.clone()];
    let (mut _w0, mut w1, mut w2) = (w0, w1, w2);
    for _ in 0..length - 1 {
        if let Some(next_words) = transition_matrix.get(&(w1.clone(), w2.clone())) {
            let next_word = next_words[rng.gen_range(0..next_words.len())].clone();
            result.push(next_word.clone());
            _w0 = w1;
            w1 = w2;
            w2 = next_word;
        }
    }
    result
}

fn read_training_data(filename: &str) -> Vec<String> {
    /*
        A function to read training data from a file.
        Note:
         1. The file should contain a list of words separated by spaces.
         2. The file should be saved in the traindata directory.
    */
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("traindata")
        .join(filename);
    fs::read_to_string(path)
        .expect("Failed to read training data file")
        .split_whitespace()
        .map(clean_word)
        .collect()
}

fn generate_passphrase(words: Vec<String>, length: usize) -> String {
    /*
        A function to generate a passphrase using a Markov chain.
    */
    let clean_words = words
        .into_iter()
        .map(|w| clean_word(&w))
        .collect::<Vec<String>>();
    let transition_matrix = create_transition_matrix(clean_words.clone());
    let start_index = rand::thread_rng().gen_range(0..clean_words.len() - 3);
    let chain = markov_chain(
        &transition_matrix,
        length,
        clean_words[start_index].clone(),
        clean_words[start_index + 1].clone(),
        clean_words[start_index + 2].clone(),
    );
    chain.join(" ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args
        .get(1)
        .expect("Please provide a passphrase length")
        .parse::<usize>()
        .expect("Length must be a positive integer");

    let words = read_training_data("kanye_verses.txt");
    let passphrase = generate_passphrase(words, length);
    println!(
        "\nYour randomly generated {} length passphrase is:\n\n{}",
        length, passphrase
    );
}
