#[derive(Debug, Clone, Copy)]
pub struct SizedString<const N: usize> {
    s: [char; N]
}
impl<const N: usize> SizedString<N> {
    pub fn new(string: &str) -> Self {
        let mut chars = [' '; N];

        for (i, c) in string.chars().take(N).enumerate() {
            chars[i] = c;
        }

        return Self {
            s: chars
        };
    }
    pub fn empty() -> Self {
        return Self {
            s: [' '; N]
        }
    }

    pub fn is_empty(&self) -> bool {
        return [' '; N] == self.s
    }

    pub fn as_str(&self) -> &str {
        return String::from_iter(self.s).as_str();
    }

    pub fn set(&mut self, new_value: &str) {
        for (i, c) in new_value.chars().take(N).enumerate() {
            self.s[i] = c;
        }
    }

    pub fn nth(&self, n: usize) -> char {
        return self.s[n];
    }

    pub fn raw(&self) -> [char; N] {
        return self.s;
    }

    pub fn count_letter(&self, letter: char) -> usize {
        return self.raw()
            .into_iter()
            .filter(|x| *x == letter)
            .count();
    }

    pub fn count_unique_letters(&self) -> usize {
        let mut counted: Vec<char> = vec![];
        for letter in self.s {
            if !counted.contains(&letter) {
                counted.push(letter);
            }
        }
        return counted.iter().count();
    }
}

impl<const N: usize> PartialEq for SizedString<N> {
    fn eq(&self, other: &Self) -> bool {
        return self.as_str() == other.as_str()
    }
}