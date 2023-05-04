use tfhe::{FheUint32};

pub struct U32InputCipher{
    inner: Vec<FheUint32>,
}



// interesting snippet from ChatGPT
// To convert str to a vector of u32 values
pub fn string_to_u32_vector(s: &str) -> Vec<u32> {
    let mut result = Vec::new();
    let bytes = s.as_bytes();

    for i in (0..bytes.len()).step_by(4) {
        let slice_end = std::cmp::min(i + 4, bytes.len());
        let bytes_slice = &bytes[i..slice_end];
        let mut value_bytes = [0u8; 4];
        value_bytes[..bytes_slice.len()].copy_from_slice(bytes_slice);
        let value = u32::from_be_bytes(value_bytes);
        result.push(value);
    }

    result
}