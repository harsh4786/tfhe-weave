use tfhe::shortint::prelude::*;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
mod constants;
use secp256k1::hashes::sha256;
//use tfhe::boolean::gen_keys;
use tfhe::shortint::parameters::Parameters;
use byteorder::{ByteOrder, LittleEndian, BigEndian};
use tfhe::core_crypto::entities::*;

fn main() {
    let (client_key, server_key) = gen_keys(Parameters::default());
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
    let msg1 = 25;
    let msg2 = 5;

    let ct1 = client_key.encrypt(msg1);
    let ct2 = client_key.encrypt(msg2);
    let ct3 = client_key.encrypt(msg1 + msg2);
    let ct4 = server_key.checked_add(&ct1, &ct2);
    if ct3.ct == ct4.unwrap().ct{
        println!("HEYYYYYYYYYYYYYYY")
    }
}

