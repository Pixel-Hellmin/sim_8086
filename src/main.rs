use std::fs;

fn main() {
    let loaded_bytes: Vec<u8> = fs::read("l_0037_single_register_mov")
        .expect("Error reading loading bytes");

    for byte in loaded_bytes {
        println!("{byte}");
    }
}
