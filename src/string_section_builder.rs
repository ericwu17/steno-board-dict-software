use core::str;
use std::collections::{HashMap, HashSet};

use crate::dict::is_good_string;

pub struct StringSectionBuilder {
    strings: HashSet<String>,
}

impl StringSectionBuilder {
    pub fn new() -> Self {
        Self {
            strings: HashSet::new(),
        }
    }

    pub fn add_string(&mut self, s: &str) {
        self.verify_string_well_formed(s);
        self.strings.insert(s.to_string());
    }

    pub fn verify_string_well_formed(&self, s: &str) {
        assert!(is_good_string(s));
    }

    pub fn get_total_string_length(&self) -> usize {
        self.strings.iter().map(|s| s.len() + 1).sum()
    }

    pub fn layout_strings(&self) -> (Vec<u8>, HashMap<String, usize>) {
        let mut all_strings: Vec<String> = self.strings.clone().into_iter().collect();
        all_strings.sort_by_key(|s| s.len());
        all_strings.reverse(); // process strings from longest to shortest

        let mut strings_in_curr_buff: Vec<String> = Vec::new();
        let mut curr_buff: Vec<u8> = Vec::new();
        let mut curr_dict: HashMap<String, usize> = HashMap::new();

        'outer: for (i, s) in all_strings.into_iter().enumerate() {
            for processed in &strings_in_curr_buff {
                if processed.ends_with(&s) {
                    let t = *curr_dict.get(processed).unwrap();
                    let new_ptr = t + (processed.len() - s.len());
                    curr_dict.insert(s, new_ptr);
                    continue 'outer;
                }
            }

            curr_dict.insert(s.clone(), curr_buff.len());
            curr_buff.extend(s.as_bytes());
            curr_buff.push(0);
            strings_in_curr_buff.push(s.clone());
            dbg!(i);
        }

        (curr_buff, curr_dict)
    }
}
