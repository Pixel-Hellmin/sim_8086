use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let loaded_bytes: Vec<u8> = fs::read(&args[1])
        .expect("Error reading loading bytes");

    for byte in loaded_bytes {
        println!("Byte {}", byte);

        for bit in (0..8).rev() {
            let bit_value = (byte >> bit) & 1;
            println!("Bit {} is {}", bit, bit_value);
        }
    }
}
