use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Read,
};

use serde::Serialize;

use crate::{string_section_builder::StringSectionBuilder, stroke::Stroke};

#[derive(Debug, Serialize)]
pub struct Dict {
    pub entries: BTreeMap<Stroke, StrokeEntryList>,
}

/// StrokeEntryList represents a non-empty list of stroke entries.
#[derive(Debug, Serialize)]
pub struct StrokeEntryList {
    entries: Vec<StrokeEntry>,
}

#[derive(Debug, Serialize)]
pub struct StrokeEntry {
    prev_strokes: Vec<Stroke>,
    string_output: StrokeStringOutput,
    flags: EntryFlags,
}

#[derive(Debug, Serialize)]
pub struct StrokeStringOutput {
    s: String,
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

    pub fn to_sections(&self) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let entries = &self.entries;
        let mut key_section = Vec::new();
        let mut val_section = Vec::new();

        let mut string_section_builder = StringSectionBuilder::new();
        for v in entries.values() {
            for e in &v.entries {
                string_section_builder.add_string(&e.string_output.s);
            }
        }
        let (string_section, dict) = string_section_builder.layout_strings();

        for (k, v) in entries.iter() {
            key_section.extend(k.to_bytes());
            let offset = val_section.len() as u32;
            key_section.extend(offset.to_le_bytes());

            val_section.extend(v.to_bytes(&dict));
        }

        println!(
            "Key section {} val section {} string section {}",
            key_section.len(),
            val_section.len(),
            string_section.len(),
        );

        (key_section, val_section, string_section)
    }
}

impl StrokeEntryList {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn to_bytes(&self, dict: &HashMap<String, usize>) -> Vec<u8> {
        let len = self.entries.len();
        assert!(len <= u16::MAX as usize);
        let len = len as u16;
        let mut res = Vec::new();

        res.extend(len.to_le_bytes());
        for e in self.entries.iter() {
            res.extend(e.to_bytes(dict))
        }

        res
    }
}

impl StrokeEntry {
    pub fn to_bytes(&self, dict: &HashMap<String, usize>) -> Vec<u8> {
        let prev_strokes_len = self.prev_strokes.len();
        assert!(prev_strokes_len <= u8::MAX as usize);
        let prev_strokes_len = prev_strokes_len as u8;
        let mut res = vec![prev_strokes_len];

        for s in self.prev_strokes.iter() {
            res.extend(s.to_bytes());
        }
        res.push(self.flags.to_byte());
        let ptr_bytes = dict.get(&self.string_output.s).unwrap().to_le_bytes();
        res.extend([ptr_bytes[0], ptr_bytes[1], ptr_bytes[2]]);
        res
    }
}

impl StrokeStringOutput {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for c in self.s.chars() {
            assert!(c.is_ascii());
            res.push(c as u8)
        }
        res.push(0);
        res
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
    pub fn to_byte(&self) -> u8 {
        self.0
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
            string_output: StrokeStringOutput { s: string_output },
            flags: EntryFlags::new(),
        };

        let e = res.entry(final_stroke).or_insert(StrokeEntryList::empty());
        e.entries.push(new_entry);
    }
    Dict { entries: res }
}

pub fn is_good_string(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii() && (c.is_whitespace() || c.is_ascii_alphabetic()))
}
