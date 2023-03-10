use std::env;
use std::fs;

const REGISTER_TABLE:[[&str; 2]; 8] = [
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

    println!("bits 16\n");

    let mut index = 0;
    while index < loaded_bytes.len() {
        let first_byte = loaded_bytes[index];
        let op = match (first_byte >> 2) & 0b00111111 {
            34 => &"mov",
            _  => &"?uop", // NOTE(Fermin): unidentified op
        };
        let d = (first_byte >> 1) & 0b00000001; 
        let w = first_byte & 0b00000001;

        let second_byte = loaded_bytes[index+1];
        let mod_field = (second_byte >> 6) & 0b00000011;
        let reg = (second_byte >> 3) & 0b00000111;
        let r_m = second_byte & 0b00000111;

        if mod_field != 3 { 
            println!("NOTE: not reg to reg op. Unsupported");
        } 

        if d == 0b00000001 {
            println!("{} {}, {}", op, REGISTER_TABLE[reg as usize][w as usize], REGISTER_TABLE[r_m as usize][w as usize]);
        } else {
            println!("{} {}, {}", op, REGISTER_TABLE[r_m as usize][w as usize], REGISTER_TABLE[reg as usize][w as usize]);
        }
        index += 2;
    }
}
