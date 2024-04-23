use serde::{
    Serialize,
    Deserialize
};

use crate::sized_string::SizedString;

const WORD_SIZE: usize = 5;
pub const GUESS_COUNT: usize = 6;

#[derive(Debug, Clone)]
pub struct WordleApp {
    word: SizedString<WORD_SIZE>,
    guesses: [SizedString<WORD_SIZE>; GUESS_COUNT]
}
impl Default for WordleApp {
    fn default() -> Self {
        return Self {
            word: SizedString::new("hello"),
            guesses: [SizedString::empty(); GUESS_COUNT]
        }
    }
}
impl WordleApp {
    pub fn new() -> Self { return Default::default(); }
}

pub struct WordleGuess<const N: usize> {
    guess: SizedString<N>
}
impl<const N: usize> WordleGuess<N> { // Checking
    pub fn check(&self, correct: SizedString<N>) -> [WordleLetterResult; N] {
        let mut results = self.check_greens(correct);


        return results;
    }

    fn check_greens(&self, correct: SizedString<N>) -> [WordleLetterResult; N] {
        let mut results = [WordleLetterResult::None; N];

        for letter in 0..N {
            results[letter] = if self.guess.nth(letter) == correct.nth(letter) {
                WordleLetterResult::Green
            }
            else {
                WordleLetterResult::Gray
            }
        };
        return results;
    }

    fn check_yellows(&self, correct: SizedString<N>, current: [WordleLetterResult; N]) -> [WordleLetterResult; N] {
        use itertools::Itertools;
        use std::collections::HashMap;

        let mut results = [WordleLetterResult::None; N];
        let unique_letters_correct = correct.count_unique_letters();
        let mut letter_counts_correct: HashMap<char, usize> = HashMap::with_capacity(5);

        // Get the amount of each letter in the correct word (to prevent duplication of
        for l in unique_letters_correct {
            let count = correct.count_letter(l.clone());
            if letter_counts_correct.contains_key(l) {
                let new_val = letter_counts_correct.get(&l).expect("failed to update correct letters hashmap") + 1;
                letter_counts_correct.insert(l.clone(), new_val);
            }
            else {
                letter_counts_correct.insert(l.clone(), 1);
            }
        }

        return results;
    }


}

#[derive(Debug, Clone, Copy)]
pub enum WordleLetterResult {
    Green,
    Yellow,
    Gray,
    None
}

impl WordleApp {

}