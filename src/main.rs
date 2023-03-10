use std::env;
use std::fs;

const REG_MEM_00_01_10:[&str; 8] = [
    "bx + si",
    "bx + di",
    "bp + si",
    "bp + di",
    "si",
    "di",
    "bp",
    "bx",
];
const REG_MEM_11:[[&str; 2]; 8] = [
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

    let mut output = String::from("bits 16\n\n");

    let mut index = 0;
    while index < loaded_bytes.len() {
        let mut index_increment = 0;

        let instruction_byte = loaded_bytes[index];
        let op = "mov";
        let w;
        let reg;

        if (instruction_byte >> 2) & 0b00111111 == 34 {
            // NOTE(Fermin): [100010 d w]
            index_increment += 2;

            let second_byte = loaded_bytes[index+1];
            w = instruction_byte & 0b00000001;
            reg = (second_byte >> 3) & 0b00000111;
            let d = (instruction_byte >> 1) & 0b00000001; 
            let mod_field = (second_byte >> 6) & 0b00000011;
            let r_m = second_byte & 0b00000111;

            let r_m_value:String = match mod_field { 
                0 => {
                    if r_m == 6 {
                        index_increment += 2;
                        "DIRECT ACCESS".to_string()
                    }
                    else {
                        format!("[{}]", REG_MEM_00_01_10[r_m as usize])
                    }
                },
                1 => {
                    index_increment += 1;
                    format!("[{} + {}]", REG_MEM_00_01_10[r_m as usize], loaded_bytes[index+2])
                },
                2 => {
                    index_increment += 2;
                    let value:i16 = ((loaded_bytes[index+3] as i16) << 8) | ((loaded_bytes[index+2] as i16));
                    format!("[{} + {}]", REG_MEM_00_01_10[r_m as usize], value)
                },
                3 => {
                    REG_MEM_11[r_m as usize][w as usize].to_string()
                },
                other => {
                    panic!("Invalid mod value: {}", other)
                },
            };

            if d == 0b00000001 {
                output += &format!("{} {}, {}\n", op, REG_MEM_11[reg as usize][w as usize], r_m_value);
            } else {
                output += &format!("{} {}, {}\n", op, r_m_value, REG_MEM_11[reg as usize][w as usize]);
            }

        } else if (instruction_byte >> 4) & 0b00001111 == 11 {
            // NOTE(Fermin): [1011 w reg]

            w = (instruction_byte >> 3) & 0b00000001;
            reg = instruction_byte & 0b00000111;

            let immediate_value:i16 = match w {
                0 => {
                    index_increment += 2;
                    loaded_bytes[index+1] as i16
                },
                1 => {
                    index_increment += 3;
                    ((loaded_bytes[index+2] as i16) << 8) | (loaded_bytes[index+1] as i16)
                },
                other => panic!("Invalid w value: {}", other)
            };

            output += &format!("{} {}, {}\n", op, REG_MEM_11[reg as usize][w as usize], immediate_value);

        } else {
            panic!("Unrecognized operation");
        }

        index += index_increment;
    }

    println!("{}", output);
}
