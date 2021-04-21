/// Binary to show how the RSA encryption works
use toy_rsa::*;

fn main() {
    let keys = genkey();
    let msg: u32 = 123456789;
    let pubk: u64 = keys.0 as u64 * keys.1 as u64;
    let cipher: u64 = encrypt(pubk, msg);
    let deciph: u32 = decrypt(keys, cipher);
    println!("msg: {}, dec: {}", msg, deciph);
}
