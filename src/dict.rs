use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Read,
};

use serde::Serialize;

use crate::stroke::Stroke;

#[derive(Debug, Serialize)]
pub struct Dict {
    entries: BTreeMap<Stroke, StrokeEntryList>,
}

/// StrokeEntryList represents a non-empty list of stroke entries.
#[derive(Debug, Serialize)]
pub struct StrokeEntryList {
    entries: Vec<StrokeEntry>,
}

#[derive(Debug, Serialize)]
pub struct StrokeEntry {
    prev_strokes: Vec<Stroke>,
    string_output: String,
    flags: EntryFlags,
}

#[derive(Debug, Serialize)]
pub enum EntryFlag {
    IsPrefix,
    IsSuffix,
    DoCapitalize,
    DoNotCapitalize,
}

#[derive(Debug, Serialize)]
pub struct EntryFlags(u8);

impl Dict {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
}

impl StrokeEntryList {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl EntryFlag {
    pub fn to_bits(&self) -> u8 {
        match self {
            EntryFlag::IsPrefix => 1 << 0,
            EntryFlag::IsSuffix => 1 << 1,
            EntryFlag::DoCapitalize => 1 << 2,
            EntryFlag::DoNotCapitalize => 1 << 3,
        }
    }
}

impl EntryFlags {
    pub fn new() -> Self {
        EntryFlags(0)
    }
    pub fn set(&mut self, f: EntryFlag) {
        self.0 |= f.to_bits();
    }
    pub fn unset(&mut self, f: EntryFlag) {
        self.0 &= !f.to_bits();
    }
}

pub fn load_dictionary_as_hashmap(path: &str) -> HashMap<Vec<Stroke>, String> {
    let mut file_contents = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut file_contents).unwrap();

    let data: HashMap<String, String> = serde_json::from_str(&file_contents).unwrap();
    data.into_iter()
        .filter_map(|(k, v)| {
            let k = Stroke::convert_str_to_stroke_vec(&k);
            k.map(|k| (k, v))
        })
        .collect()
}

pub fn generate_dictionary(path: &str) -> Dict {
    let dictionary_hashmap = load_dictionary_as_hashmap(path);

    let mut res = BTreeMap::new();

    for (mut stroke_vec, string_output) in dictionary_hashmap.into_iter() {
        let final_stroke = stroke_vec.pop().unwrap();
        let remaining_strokes = stroke_vec;

        if !is_good_string(&string_output) {
            continue;
        }
        
        let new_entry = StrokeEntry {
            prev_strokes: remaining_strokes,
            string_output,
            flags: EntryFlags::new(),
        };

        let e = res.entry(final_stroke).or_insert(StrokeEntryList::empty());
        e.entries.push(new_entry);
    }
    Dict {
        entries: res
    }
}

pub fn is_good_string(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii() && (c.is_whitespace() || c.is_ascii_alphabetic()))
}
