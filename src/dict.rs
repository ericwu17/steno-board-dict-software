use std::{collections::HashMap, fs::File, io::Read};

use crate::stroke::Stroke;

pub fn load_dictionary(path: &str) {
    let mut file_contents = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut file_contents).unwrap();

    let data: HashMap<String, String> = serde_json::from_str(&file_contents).unwrap();
    let t: HashMap<Vec<Stroke>, String> = data
        .into_iter()
        .filter_map(|(k, v)| {
            let k = Stroke::convert_str_to_stroke_vec(&k);
            k.map(|k| (k, v))
        })
        .collect();

    println!("done");
    println!("{}", t.len());
}
