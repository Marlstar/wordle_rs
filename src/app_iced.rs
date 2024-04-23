use serde::{
    Serialize,
    Deserialize
};

use crate::sized_string::SizedString;

const WORD_SIZE: usize = 5;
pub const GUESS_COUNT: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordleApp {
    word: SizedString<WORD_SIZE>,
    guesses: [SizedString<WORD_SIZE>; GUESS_COUNT]
}
impl Default for WordleApp {
    fn default() -> Self {
        return Self {
            word: SizedString::new("hello"),
            guesses
        }
    }
}
impl WordleApp {
    pub fn new() -> Self {
        return Default::default();
    }
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

        for letter in ..N {
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

        let mut results = [WordleLetterResult::None; N];
        let unique_letters_self = self.guess.raw().iter().unique();
        let letter_counts_self: Vec<(char, usize)> = vec![];



        return results;
    }

    fn count_letter(word: SizedString<N>, letter: char) {
        let mut x = 0;
        for l in word.raw() {
            if l == letter { x += 1; }
        }
    }
}
pub enum WordleLetterResult {
    Green,
    Yellow,
    Gray,
    None
}

impl WordleApp {

}