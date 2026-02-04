use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    fs::File,
    hash::Hash,
    io::Read,
};

fn main() {
    load_dictionary("example.json");
}

pub fn load_dictionary(path: &str) {
    let mut file_contents = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut file_contents).unwrap();

    let data: HashMap<String, String> = serde_json::from_str(&file_contents).unwrap();
    let t: HashMap<Vec<Stroke>, String> = data
        .into_iter()
        .filter_map(|(k, v)| {
            let k = convert_str_to_stroke_vec(&k);
            match k {
                Some(k) => Some((k, v)),
                None => None,
            }
        })
        .collect();

    println!("done");
    println!("{}", t.len());
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Stroke(u32);

impl Stroke {
    pub fn assert_is_valid(&self) {
        // The representation of a byte uses the lowest 3 bytes of the u32.
        let n = self.0;
        assert!(n & 0xFF00_0000 == 0);
    }

    pub fn to_int(&self) -> u32 {
        self.0
    }
    pub fn try_stroke_str_to_int(s: &str) -> Option<u32> {
        if s.is_empty() {
            return None;
        }
        let mut stroke_chars: VecDeque<char> = s.chars().collect();
        let steno_stroke_order = "#ZSTKPWHRAO*EUFRPBLGTSDZ";
        let steno_stroke_order_chars: Vec<char> = steno_stroke_order.chars().collect();
        let n = steno_stroke_order_chars.len();

        let mut current_pos = 0;
        let mut result = 0;

        while stroke_chars.len() > 0 {
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

        let has_hyphen = if stroke & Stroke::stroke_str_to_int("ZSTKPWHR") == stroke {
            false
        } else if stroke & Stroke::stroke_str_to_int("AO*EU") != 0 {
            false
        } else {
            true
        };

        let mut res_string = String::new();

        let steno_stroke_order = "#ZSTKPWHRAO*EUFRPBLGTSDZ";
        let steno_stroke_order_chars: Vec<char> = steno_stroke_order.chars().collect();
        for i in 0..=23 {
            if stroke & (1 << i) != 0 {
                res_string.push(steno_stroke_order_chars[i]);
            }

            if i == 11 && has_hyphen {
                res_string.push('-');
            }
        }

        write!(f, "{}", res_string)
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
        assert_eq!(Stroke::stroke_str_to_int("Z"), 2);
        assert_eq!(Stroke::stroke_str_to_int("S"), 4);
        assert_eq!(Stroke::stroke_str_to_int("#"), 1);
        assert_eq!(Stroke::stroke_str_to_int("T"), 8);
        assert_eq!(Stroke::stroke_str_to_int("-T"), 1 << 20);

        assert_eq!(Stroke::stroke_str_to_int("-Z"), 1 << 23);

        assert_eq!(
            Stroke::stroke_str_to_int("-PBT"),
            1 << 16 | 1 << 17 | 1 << 20
        );
    }

    #[test]
    fn all_nums_to_stroke_and_back() {
        for i in 1..(1 << 24) {
            let stroke = Stroke(i);
            let s = format!("{stroke}");

            let n = Stroke::stroke_str_to_int(&s);
            assert_eq!(n, i);
        }
    }
}
