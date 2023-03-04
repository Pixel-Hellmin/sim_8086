use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let loaded_bytes: Vec<u8> = fs::read(&args[1])
        .expect("Error reading file");

    static REGISTER_TABLE:[[&str; 2]; 8] = [["al", "ax"], ["cl", "cx"], ["dl", "dx"], ["bl", "bx"], ["ah", "sp"], ["ch", "bp"], ["dh", "si"], ["bh", "di"], ];
    let mut result = String::new();

    for instruction in (0..loaded_bytes.len()).step_by(2) {
        let mut result_operation = &"";

        let first_byte = loaded_bytes[instruction];
        // NOTE(Fermin) 0b00100010 == mov == 34, in decimal
        let operation = (first_byte >> 2) & 0b00111111;
        let d = (first_byte >> 1) & 0b00000001; 
        let w = first_byte & 0b00000001;

        let second_byte = loaded_bytes[instruction+1];
        let mod_field = (second_byte >> 6) & 0b00000011;
        let reg = (second_byte >> 3) & 0b00000111;
        let r_m = second_byte & 0b00000111;

        if operation == 34 { result_operation = &"mov"; } 
        else { 
            println!("PANIC: Not mov operation");
            break;
        }
        if mod_field != 3 { 
            println!("PANIC: We only support register to register operations");
            break;
        } 

        if d == 0b00000001 {
            result += &format!("{} {}, {}\n", result_operation, REGISTER_TABLE[reg as usize][w as usize], REGISTER_TABLE[r_m as usize][w as usize]);
        } else {
            result += &format!("{} {}, {}\n", result_operation, REGISTER_TABLE[r_m as usize][w as usize], REGISTER_TABLE[reg as usize][w as usize]);
        }

        /*
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
