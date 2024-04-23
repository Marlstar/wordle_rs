#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SizedString<const N: usize> {
    s: [char; N]
}
impl<const N: usize> SizedString<N> {
    pub fn new(string: &str) -> Self<N> {
        let mut chars = [' '; N];

        for (i, c) in string.chars().take(N).enumerate() {
            chars[i] = c;
        }

        return Self {
            s: chars
        };
    }

    pub fn is_empty(&self) -> bool {
        return [' '; N] == self.s
    }

    pub fn as_str(&self) -> &str {
        let s = self.s.iter().collect();
        return s;
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
}

impl<const N: usize> PartialEq for SizedString<N> {
    fn eq(&self, other: &Self) -> bool {
        return self.as_str() == other.as_str()
    }
}