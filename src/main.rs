pub mod dict;
pub mod stroke;

fn main() {
    let d = dict::generate_dictionary("example.json");
    let str = serde_json::to_string_pretty(&d).unwrap();

    println!("{str}");
}
