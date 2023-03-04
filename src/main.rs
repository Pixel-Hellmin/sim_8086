use std::env;
use std::fs;

static REGISTER_TABLE:[[&str; 2]; 8] = [
    ["al", "ax"],
    ["cl", "cx"],
    ["dl", "dx"],
    ["bl", "bx"],
    ["ah", "sp"],
    ["ch", "bp"],
    ["dh", "si"],
    ["bh", "di"],
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let loaded_bytes: Vec<u8> = fs::read(&args[1])
        .expect("Error reading file");

    let mut result = String::from("bits 16\n\n");

    for instruction in (0..loaded_bytes.len()).step_by(2) {
        let first_byte = loaded_bytes[instruction];
        let op = match (first_byte >> 2) & 0b00111111 {
            34 => &"mov",
            _  => &"?uop", // NOTE(Fermin): unidentified op
        };
        let d = (first_byte >> 1) & 0b00000001; 
        let w = first_byte & 0b00000001;

        let second_byte = loaded_bytes[instruction+1];
        let mod_field = (second_byte >> 6) & 0b00000011;
        let reg = (second_byte >> 3) & 0b00000111;
        let r_m = second_byte & 0b00000111;

        if mod_field != 3 { 
            println!("NOTE: not reg to reg op. Unsupported");
        } 

        if d == 0b00000001 {
            result += &format!("{} {}, {}\n", op, REGISTER_TABLE[reg as usize][w as usize], REGISTER_TABLE[r_m as usize][w as usize]);
        } else {
            result += &format!("{} {}, {}\n", op, REGISTER_TABLE[r_m as usize][w as usize], REGISTER_TABLE[reg as usize][w as usize]);
        }

        /* DEBUG: print bits
        for bit in 0..8 {
            let bit_value = (first_byte >> bit) & 1;
            println!("Bit {} is {}", bit, bit_value);
        }
        for bit in 0..8 {
            let bit_value = (second_byte >> bit) & 1;
            println!("Bit {} is {}", bit, bit_value);
        }
        */
    }

    println!("{}", result);
}
