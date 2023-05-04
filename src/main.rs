use tfhe::prelude::*;
// use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
mod constants;
mod u32_ciphers;
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
    

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());
    let sig = secret_key.sign_ecdsa(message).to_string();
    let u32_text = string_to_u32_vector(&sig.as_str());
    let e_vec = u32_text.into_iter().map(|i| FheUint32::try_encrypt(i, &client_key).unwrap()).collect::<Vec<_>>();
   
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

