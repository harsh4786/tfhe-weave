use tfhe::prelude::*;
// use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
mod constants;
mod u32_ciphers;
use u32_ciphers::{U32InputCipher};
use constants::{IV, MSG_PERMUTATION};
use secp256k1::rand::rngs::OsRng;
use u32_ciphers::string_to_u32_vector;
use secp256k1::hashes::sha256;// use secp256k1::hashes::sha256;
//use tfhe::boolean::gen_keys;
use tfhe::shortint::parameters::Parameters;
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint32, FheUint8};
// use byteorder::{ByteOrder, LittleEndian, BigEndian};
// use tfhe::core_crypto::entities::*;
// use tfhe::boolean::ciphertext::Ciphertext;

fn main() {
    let config =  ConfigBuilder::all_disabled().enable_default_uint32().build();
    let (client_key, server_key) = generate_keys(config);
    
    let encrypted_iv = U32InputCipher::encrypt(IV.to_vec(), &client_key);
   // let secp = Secp256k1::new();
   // let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
   // let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());
    //let sig = secret_key.sign_ecdsa(message).to_string();
  //  let u32_text = string_to_u32_vector(&sig.as_str());
    //let e_vec = u32_text.into_iter().map(|i| FheUint32::try_encrypt(i, &client_key).unwrap()).collect::<Vec<FheUint32>>();
   
   // let fhe_pubkey = PublicKey::new(&client_key);
    // let secp = Secp256k1::new();
    // let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
   //  let fhe_secret = fhe_pubkey.encrypt(secret_key);
   
    // if LittleEndian::read_u64(&secret_key[0..8]) == 0 {
    //     println!("System uses little-endian byte order");
    // } else if BigEndian::read_u64(&secret_key[0..8]) == 0 {
    //     println!("System uses big-endian byte order");
    // } else {
    //     println!("Could not determine byte order");
    // }
    // let msg1 = 25;
    // let msg2 = 5;
    //let scalar =  1000000000000000000u64;
   // print!("{:?}", u8scala);

    // let ct1 = client_key.encrypt(msg1);
    // let ct2 = client_key.encrypt(msg2);
    // let ct3 = client_key.encrypt(msg1 + msg2);
    // let ct4 = server_key.checked_add(&ct1, &ct2);
    // if ct3.ct == ct4.unwrap().ct{
    //     println!("HEYYYYYYYYYYYYYYY");
    // }
   // let test = server_key.unchecked_scalar_add(&ct3, scalar);
   // server_key.smart_scalar_add(ct, scalar)
}

fn rotate_right(x: FheUint32, amount: u32) -> FheUint32 {
    (x.clone() >> amount) | (x.clone() << (32u32 - amount))
}

fn g(state: &mut [FheUint32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    state[a] = state[a].clone() + state[b].clone() +  mx;
    state[d] = rotate_right(state[d].clone() ^ state[a].clone(),16);
    state[c] = state[c].clone() + state[d].clone();
    state[b] = rotate_right(state[b].clone() ^ state[c].clone(), 12);
    state[a] = state[a].clone() + state[b].clone() + my;
    state[d] = rotate_right(state[d].clone() ^ state[a].clone(), 8);
    state[c] = state[c].clone() + state[d].clone();
    state[b] = rotate_right(state[b].clone() ^ state[c].clone(),7);
}

fn round(state: &mut [FheUint32; 16], m: &[u32; 16]) {
    // Mix the columns.
    g(state, 0, 4, 8, 12, m[0], m[1]);
    g(state, 1, 5, 9, 13, m[2], m[3]);
    g(state, 2, 6, 10, 14, m[4], m[5]);
    g(state, 3, 7, 11, 15, m[6], m[7]);
    // Mix the diagonals.
    g(state, 0, 5, 10, 15, m[8], m[9]);
    g(state, 1, 6, 11, 12, m[10], m[11]);
    g(state, 2, 7, 8, 13, m[12], m[13]);
    g(state, 3, 4, 9, 14, m[14], m[15]);
}

pub fn fhe_blake3(msg: &str) -> [FheUint32; 32]{
    todo!()
}

// forming a new array of values that are modified from the original "m" array passed to the permute function
// since there are 16 values in PERMUTATION array [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]
// the for loop starts at index 0 and the value from PERMUTATION array is "2" so it fetches the value that is 
// present in the "m" array at index 2 and sets it to our permuted array which was initialized with [0, 0, 0, 0, 0, 0, 0, 0, 0, 0....upto 16 zeroes]
// and returns the permuted array formed from the input "m" array but directed by the PERMUTATION array.
fn permute(m: &mut [FheUint32; 16]) {
    let mut permuted = [FheUint32::encrypt_trivial(0, client_key).unwrap().clone(); 16];
    for i in 0..16 {
        permuted[i] = m[MSG_PERMUTATION[i]];
    }
    *m = permuted;
}
