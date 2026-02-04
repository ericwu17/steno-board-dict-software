pub mod dict;
pub mod stroke;

fn main() {
    let d = dict::generate_dictionary("example.json");
    let my_bytes = d.to_bytes();
    println!("total section has {} bytes", my_bytes.len());
}
