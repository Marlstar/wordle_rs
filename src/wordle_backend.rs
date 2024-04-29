use std::fmt::Formatter;
use crate::sized_string::SizedString;

const WORD_SIZE: usize = 5;
pub const GUESS_COUNT: usize = 6;

pub mod prelude {
    pub use super::{
        WordleBackend,
        WordleGuess,
        WordleLetterResult,
        WordleLetterColour
    };
}

#[derive(Debug, Clone)]
pub struct WordleBackend {
    word: SizedString<WORD_SIZE>,
    guesses: [SizedString<WORD_SIZE>; GUESS_COUNT]
}
impl Default for WordleBackend {
    fn default() -> Self {
        return Self {
            word: SizedString::new("hello"),
            guesses: [SizedString::empty(); GUESS_COUNT]
        }
    }
}
impl WordleBackend {
    pub fn new() -> Self { return Default::default(); }
    pub fn from_starter_word(word: SizedString<WORD_SIZE>) -> Self {return Self {word, ..Default::default()}}
}

pub struct WordleGuess<const N: usize> {
    guess: SizedString<N>
}
impl<const N: usize> WordleGuess<N> {
    pub fn new(string: SizedString<N>) -> Self {
        return Self {
            guess: string
        }
    }
}
impl<const N: usize> WordleGuess<N> { // Checking
    pub fn check(&self, correct: SizedString<N>) -> [WordleLetterResult; N] {
        use std::collections::HashMap;

        let change_by_x = |hash_map: &mut HashMap<char, isize>, c: &char, x: isize| {
            if hash_map.contains_key(c) {
                if hash_map.get(c).unwrap_or(&0) > &0 {
                    hash_map.insert(
                        c.clone(),
                        hash_map.get(c).expect("failed to get value in hashmap") + x
                    );
                };
            }
            else {
                hash_map.insert(c.clone(), x);
            };
        };

        let mut results_raw = [WordleLetterColour::None; N];
        let mut letter_counts_correct: HashMap<char, isize> = HashMap::with_capacity(5);

        // Get the amount of each letter in the correct word (to prevent duplication of yellows later)
        for l in correct.raw() {
            change_by_x(&mut letter_counts_correct, &l, 1);
        }

        // Go through each letter in the guess and check if they are green
        for i in 0..N {
            if self.guess.nth(i) == correct.nth(i) {
                results_raw[i] = WordleLetterColour::Green;
                change_by_x(&mut letter_counts_correct, &self.guess.nth(i), -1);
            }
        }

        // Get the yellow ones!
        for i in 0..N {
            if results_raw[i] != WordleLetterColour::Green { // If it is not already green
                if letter_counts_correct.get(&self.guess.nth(i)).unwrap_or(&0) > &0 {
                    change_by_x(&mut letter_counts_correct, &self.guess.nth(i), -1);
                    results_raw[i] = WordleLetterColour::Yellow;
                }
                else {
                    results_raw[i] = WordleLetterColour::Gray;
                }
            }
        }

        let mut results: [WordleLetterResult; N] = [WordleLetterResult::temp_empty(); N];
        for i in 0..N {
            results[i] = WordleLetterResult::from(self.guess.nth(i).clone(), results_raw[i])
        }

        return results;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordleLetterResult {
    pub letter: char,
    pub result: WordleLetterColour
}
impl WordleLetterResult {
    pub fn from(letter: char, result: WordleLetterColour) -> Self {
        return Self {
            letter,
            result
        }
    }

    pub fn temp_empty() -> Self {
        return Self {
            letter: ' ',
            result: WordleLetterColour::None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordleLetterColour {
    Green,
    Yellow,
    Gray,
    None
}

impl WordleBackend {

}