use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let loaded_bytes: Vec<u8> = fs::read(&args[1])
        .expect("Error reading file");

    let mut result = String::new();

    for instruction in (0..loaded_bytes.len()).step_by(2) {
        println!("instructions {}", instruction);
        
        let mut result_operation = &"";

        let first_byte = loaded_bytes[instruction];
        println!("first byte {}", first_byte);
        // NOTE(Fermin) 0b00100010 == mov == 34, in decimal
        let operation = (first_byte >> 2) & 0b00111111;
        println!("operation {}", operation);
        let d = (first_byte >> 1) & 0b00000001; 
        println!("d {}", d);
        let w = first_byte & 0b00000001;
        println!("w {}", w);

        let second_byte = loaded_bytes[instruction+1];
        println!("second byte {}", second_byte);
        let mod_field = (second_byte >> 6) & 0b00000011;
        println!("mod {}", mod_field);
        let reg = (second_byte >> 3) & 0b00000111;
        println!("reg {}", reg);
        let r_m = second_byte & 0b00000111;
        println!("r_m {}", r_m);

        if operation == 34 { result_operation = &"mov"; } 
        else { 
            println!("PANIC: Not mov operation");
            break;
        }
        if mod_field != 3 { 
            println!("PANIC: We only support register to register operations");
            break;
        } 

        let reg_res: &str = match reg {
            0b00000000 => {
                if w == 0b00000000 {"al"} else {"ax"}
            },
            0b00000001 => {
                if w == 0b00000000 {"cl"} else {"cx"}
            },
            0b00000010 => {
                if w == 0b00000000 {"dl"} else {"dx"}
            },
            0b00000011 => {
                if w == 0b00000000 {"bl"} else {"bx"}
            },
            0b00000100 => {
                if w == 0b00000000 {"ah"} else {"sp"}
            },
            0b00000101 => {
                if w == 0b00000000 {"ch"} else {"bp"}
            },
            0b00000110 => {
                if w == 0b00000000 {"dh"} else {"si"}
            },
            0b00000111 => {
                if w == 0b00000000 {"bh"} else {"di"}
            },
            _ => "reg not found"
        };

        let r_m_res: &str = match r_m {
            0b00000000 => {
                if w == 0b00000000 {"al"} else {"ax"}
            },
            0b00000001 => {
                if w == 0b00000000 {"cl"} else {"cx"}
            },
            0b00000010 => {
                if w == 0b00000000 {"dl"} else {"dx"}
            },
            0b00000011 => {
                if w == 0b00000000 {"bl"} else {"bx"}
            },
            0b00000100 => {
                if w == 0b00000000 {"ah"} else {"sp"}
            },
            0b00000101 => {
                if w == 0b00000000 {"ch"} else {"bp"}
            },
            0b00000110 => {
                if w == 0b00000000 {"dh"} else {"si"}
            },
            0b00000111 => {
                if w == 0b00000000 {"bh"} else {"di"}
            },
            _ => "reg not found"
        };

        if d == 0b00000001 {
            result += &format!("{} {}, {}\n", result_operation, reg_res, r_m_res);
        } else {
            result += &format!("{} {}, {}\n", result_operation, r_m_res, reg_res);
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
