use std::fs;

fn main() {
    let loaded_bytes: Vec<u8> = fs::read("l_0037_single_register_mov")
        .expect("Error reading loading bytes");

    for byte in loaded_bytes {
        println!("Byte {}", byte);
        for bit in 0..8 {
            let bit_value = (byte >> bit) & 1;
            println!("Bit {} is {}", bit, bit_value);
        }
    }
}
