#![allow(clippy::new_without_default)]
pub mod dict;
pub mod string_section_builder;
pub mod stroke;

fn main() {
    let d = dict::generate_dictionary("example.json");
    let (v1, v2, v3) = d.to_sections();
    println!("total section has {} bytes", v1.len() + v2.len() + v3.len());
}
