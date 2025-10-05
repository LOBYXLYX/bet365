use std::f64;
use crate::bytecode::bytearray::{decode_bytecode};
use crate::disassembler::instructions::Instructions;

pub struct Disassembler {
    bytearray: Vec<u8>,
    ptr: usize,
    pub registers: Vec<String>,
    pub trace: Vec<String>
}


fn left_pad(input: &str, target_length: usize, pad_char: Option<&str>) -> String {
    let pad_char = pad_char.unwrap_or(" ");
    if input.len() >= target_length {
        return input.to_string();
    }

    let mut pad = pad_char.to_string();
    let pad_length = target_length - input.len();

    if pad_length > pad.len() {
        let repeat_count = pad_length / pad.len();
        pad = pad.repeat(repeat_count + 1);
    }

    let final_pad = &pad[..pad_length];
    format!("{}{}", final_pad, input)
}

impl Disassembler {
    pub fn new(bytecode: String) -> Self {
        let bytearray: Vec<u8> = decode_bytecode(bytecode);

        Self {
            bytearray: bytearray,
            ptr: 0,
            registers: vec![String::from("_free_reg_"); 256],
            trace: Vec::new()
        }
    }

    pub fn get_byte(&mut self) -> u8 {
        let byte = self.bytearray[self.ptr];
        self.ptr += 1;
        byte
    }

    fn get_pointer_byte(&mut self) -> u32 {
        let byte1 = (self.bytearray[self.ptr] as u32) << 8;
        self.ptr += 1;
        let byte2 = byte1 | (self.bytearray[self.ptr] as u32);
        self.ptr += 1;
        byte2
    }

    pub fn decode_value(&mut self) -> String {
        let string_len: u32 = self.get_pointer_byte();
        let mut string: String = String::new();

        for _ in 0..string_len {
            let byte = self.get_byte();

            let c: char = std::char::from_u32((byte as u32) ^ 50).unwrap();
            string.push_str(&c.to_string());
        }
        string
    }

    pub fn read_double(&mut self) -> f64 {
        let mut bit_string = String::new();
        for _ in 0..8 {
            let byte: u8 = self.get_byte();
            let bits: String = format!("{:b}", byte);
            bit_string.push_str(&left_pad(&bits, 8, Some("0")));
        }

        let sign: f64 = if &bit_string[0..1] == "1" { -1.0 } else { 1.0 };

        let exponent_bits: &str = &bit_string[1..12];
        let mut exponent: i32 = i32::from_str_radix(exponent_bits, 2).unwrap();

        let mantissa_bits: &str = &bit_string[12..];

        let mantissa_string: String;
        if exponent == 0 {
            if !mantissa_bits.contains('1') {
                return 0.0;
            }
            exponent = -1022;
            mantissa_string = format!("0{}", mantissa_bits);
        } else {
            exponent -= 1023;
            mantissa_string = format!("1{}", mantissa_bits);
        }

        let mut mantissa = 0.0;
        let mut frac = 1.0;
        for c in mantissa_string.chars() {
            mantissa += frac * c.to_digit(10).unwrap() as f64;
            frac /= 2.0;
        }

        sign * mantissa * f64::powi(2.0, exponent)
    }

    pub fn get_int24(&mut self) -> u32 {
        let byte: u32 = (self.get_byte() as u32) << 24 | (self.get_byte() as u32) << 16 | (self.get_byte() as u32) << 8 | (self.get_byte() as u32);
        byte
    }

    pub fn execute(&mut self) {
        let instructions = Instructions::get_instructions();
        while self.ptr < self.bytearray.len() {
            let offset = self.bytearray[self.ptr];
            self.ptr += 1;

            if let Some(opcode) = instructions.get(&offset) {
                opcode(self);

                let last_instr = self.trace[self.trace.len() - 1].clone();
                let new_instr = format!("0x{}    {}", self.ptr, last_instr);
                println!("{}", new_instr);
            } else {
                panic!("{}", format!("Unknown Opcode: {}", offset))
            }
        }
    }    
}