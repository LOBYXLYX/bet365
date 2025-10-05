use base64::{engine::general_purpose, Engine as _};

pub fn decode_bytecode(mut b64string: String) -> Vec<u8> {
    b64string.retain(|c| !c.is_whitespace());
    general_purpose::STANDARD.decode(b64string).unwrap()
}