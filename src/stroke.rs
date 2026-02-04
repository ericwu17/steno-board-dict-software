use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    hash::Hash,
};

use serde::Serialize;

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Stroke(u32);

impl Stroke {
    pub fn assert_is_valid(&self) {
        // The representation of a byte uses the lowest 3 bytes of the u32.
        let n = self.0;
        assert!(n & 0xFF80_0000 == 0);
    }

    pub fn to_int(&self) -> u32 {
        self.0
    }
    pub fn try_stroke_str_to_int(s: &str) -> Option<u32> {
        if s.is_empty() {
            return None;
        }
        let mut stroke_chars: VecDeque<char> = s.chars().collect();
        let steno_stroke_order = "#STKPWHRAO*EUFRPBLGTSDZ";
        let steno_stroke_order_chars: Vec<char> = steno_stroke_order.chars().collect();
        let n = steno_stroke_order_chars.len();

        let mut current_pos = 0;
        let mut result = 0;

        while !stroke_chars.is_empty() {
            let c = stroke_chars.pop_front().unwrap();
            if c == '-' {
                if current_pos < 11 {
                    // when we see a '-', set the current position to be the middle
                    current_pos = 11;
                }
            } else {
                while current_pos < n && steno_stroke_order_chars[current_pos] != c {
                    current_pos += 1
                }
                if current_pos == n {
                    // ran off the end when parsing stroke string
                    return None;
                }
                result |= 1 << current_pos;
            }
        }

        Some(result)
    }

    pub fn stroke_str_to_int(s: &str) -> u32 {
        Self::try_stroke_str_to_int(s).unwrap()
    }

    pub fn hash(&self) -> u32 {
        // https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key
        let mut x = self.0;
        x = ((x >> 16) ^ x) * 0x45d9f3bu32;
        x = ((x >> 16) ^ x) * 0x45d9f3bu32;
        x = (x >> 16) ^ x;
        x
    }

    pub fn convert_str_to_stroke_vec(s: &str) -> Option<Vec<Stroke>> {
        let mut res = Vec::new();
        for frag in s.split("/") {
            if let Some(s) = Stroke::try_stroke_str_to_int(frag) {
                res.push(Stroke::from(s))
            } else {
                return None;
            }
        }

        Some(res)
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        let all_bytes = self.0.to_le_bytes();
        [all_bytes[0], all_bytes[1], all_bytes[2]]
    }
}

impl Debug for Stroke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.0;
        write!(f, "Stroke: {n:#032b}")?; // 32-bit binary number
        Ok(())
    }
}

impl Display for Stroke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Displays the stroke as a string...
        // Eg: "STKRO*EUD", "STKR-T", "-G", "S" (leading S)
        // If the stroke contains only strokes from the LHS of the keyboard, there is no hyphen,
        // Or if the stroke contains any of the vowels or asterisk, there is no hyphen.
        let stroke = self.0;

        let has_hyphen = !(stroke & Stroke::stroke_str_to_int("STKPWHR") == stroke
            || stroke & Stroke::stroke_str_to_int("AO*EU") != 0);

        let mut res_string = String::new();

        let steno_stroke_order = "#STKPWHRAO*EUFRPBLGTSDZ";
        let steno_stroke_order_chars: Vec<char> = steno_stroke_order.chars().collect();
        for (i, char_item) in steno_stroke_order_chars.iter().enumerate() {
            if stroke & (1 << i) != 0 {
                res_string.push(*char_item);
            }

            if i == 11 && has_hyphen {
                res_string.push('-');
            }
        }

        write!(f, "{}", res_string)
    }
}

impl Serialize for Stroke {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl From<u32> for Stroke {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<&str> for Stroke {
    fn from(value: &str) -> Self {
        Self(Self::stroke_str_to_int(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stroke_parsing_test() {
        assert_eq!(Stroke::stroke_str_to_int("S"), 2);
        assert_eq!(Stroke::stroke_str_to_int("#"), 1);
        assert_eq!(Stroke::stroke_str_to_int("T"), 4);
        assert_eq!(Stroke::stroke_str_to_int("-T"), 1 << 19);

        assert_eq!(Stroke::stroke_str_to_int("-Z"), 1 << 22);

        assert_eq!(
            Stroke::stroke_str_to_int("-PBT"),
            1 << 15 | 1 << 16 | 1 << 19
        );
    }

    #[test]
    fn all_nums_to_stroke_and_back() {
        for i in 1..(1 << 23) {
            let stroke = Stroke(i);
            let s = format!("{stroke}");

            let n = Stroke::stroke_str_to_int(&s);
            assert_eq!(n, i);
        }
    }
}
