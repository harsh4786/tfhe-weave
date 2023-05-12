use tfhe::{
    {FheUint32, ClientKey, FheUint8 }, prelude::FheTryEncrypt,
};
use crate::constants::{ BLOCK_LEN};

pub struct U32InputCipher{
    inner: Vec<FheUint32>,
}
impl U32InputCipher{
   pub fn encrypt(v: Vec<u32>, k: &ClientKey ) -> Self{
        let e_vec = v
        .into_iter()
        .map(|i| FheUint32::try_encrypt(i, k).unwrap())
        .collect::<Vec<FheUint32>>();
        Self { inner: e_vec }
    }
}

pub struct ChunkState {
    chaining_value: [FheUint32; 8],
    chunk_counter: u64,
    block: [FheUint8; BLOCK_LEN],
    block_len: u8,
    blocks_compressed: u8,
    flags: u32,
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