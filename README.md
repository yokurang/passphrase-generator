---

# Markov Phrase Generator

The Markov Phrase Generator is a simple passphrase generator that utilizes Markov chains to generate passphrases based on a given training text. This project demonstrates how to apply Markov chains for text generation by creating a transition matrix from the training data and then using it to generate new phrases.

## Core Concepts

### Transition Matrix Creation

The transition matrix is a fundamental component of this project. It maps each pair of consecutive words (bigrams) from the training text to a vector of possible subsequent words. This matrix is created by the `create_transition_matrix` function, which scans through the training text and records the occurrence of each triplet (trigram), storing the third word as a potential following word for the first two words.

```rust
fn create_transition_matrix(words: Vec<String>) -> HashMap<(String, String), Vec<String>> {
    let mut transition_matrix = HashMap::new();
    for window in words.windows(3) {
        if let [w0, w1, w2] = &window {
            let entry = transition_matrix.entry((w0.clone(), w1.clone())).or_insert_with(Vec::new);
            entry.push(w2.clone());
        }
    }
    transition_matrix
}
```

### Markov Chain for Passphrase Generation

With the transition matrix in place, the `markov_chain` function generates a sequence of words to form a passphrase. Starting with an initial triplet, it selects the next word based on the transition matrix and repeats this process to achieve the desired passphrase length.

```rust
fn markov_chain(
    transition_matrix: &HashMap<(String, String), Vec<String>>,
    length: usize,
    w0: String,
    w1: String,
    w2: String,
) -> Vec<String> {
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
```

### Generating the Passphrase

The `generate_passphrase` function orchestrates the process, reading the training data, cleaning it, creating the transition matrix, and finally invoking the Markov chain function to generate and return the passphrase.

```rust
fn generate_passphrase(words: Vec<String>, length: usize) -> String {
    let clean_words = words.into_iter().map(clean_word).collect::<Vec<String>>();
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
```

Certainly! Here is an expanded section of the README that includes instructions on how to install Rust and Cargo, followed by how to run the Markov Phrase Generator project.

## Prerequisites

Before you can run the Markov Phrase Generator, you need to have Rust and Cargo installed on your system. Rust is a fast and memory-efficient programming language, and Cargo is its package manager and build system.

### Installing Rust and Cargo

Follow these steps to install Rust and Cargo on your system:

1. **Open a Terminal or Command Prompt.**

2. **Install Rustup:**
   Rustup is an installer for the Rust programming language. Installing Rustup will also install Rust and Cargo. Run the following command in your terminal:

   - On Unix-based systems (Linux, macOS), use:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) from the official Rust website.

3. **Follow the on-screen instructions:**
   The installer will guide you through the installation process. Make sure to proceed with the default installation unless you have specific customizations in mind.

4. **Restart your Terminal or Command Prompt:**
   This ensures that the changes to your environment variables are applied.

5. **Verify the Installation:**
   Once the installation is complete, you can verify it by running:
   ```bash
   rustc --version
   ```
   This command should return the version of Rust that was installed, indicating that Rust and Cargo are correctly installed on your system.

## Running the Project

With Rust and Cargo installed, you can now run the Markov Phrase Generator project.

1. **Clone the Project:**
   If the project is hosted on a version control system like GitHub, clone it to your local machine using:
   ```bash
   git clone https://github.com/yokurang/passphrase-generator.git 
   ```

2. **Navigate to the Project Directory:**
   Change into the project directory using:
   ```bash
   cd markov-phrasegen
   ```

3. **Build the Project:**
   Compile the project with Cargo by running:
   ```bash
   cargo build
   ```

4. **Run the Project:**
   After building, run the executable with an optional argument specifying the desired length of the passphrase. If no length is provided, a default value will be used. For example, to generate a passphrase of length 12, use:
   ```bash
   cargo run -- 12
   ```
This command generates a passphrase of length 12 based on the patterns found in "kanye_verses.txt".

Note that the training text "kanye_verses.txt" comes from the Kaggle dataset ["Kanye West Verses"](https://www.kaggle.com/viccalexander/kanyewestverses) and contains a collection of verses from Kanye West's songs.

## Applications

The Markov Phrase Generator can be used to generate passphrases. This is particularly useful for creating memorable and secure passphrases that are not easily guessable. The generator can be trained on a variety of texts, such as song lyrics, books, or articles, to create passphrases that are unique and relevant to the training data.

Some applications where this might be useful include:
 1. Generating secure passphrases for blockchain wallets
 2. Generating unique passphrases for encryption keys

---
