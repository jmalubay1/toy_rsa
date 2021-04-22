/// Binary to show how the RSA encryption works
/// Generates a random integer as the messege and runs through RSA
use rand::Rng;
use toy_rsa::*;

fn main() {
    let keys = genkey();
    let pubk: u64 = (keys.0 as u64) * (keys.1 as u64);
    let msg: u32 = rand::thread_rng().gen::<u32>();
    let cipher: u64 = encrypt(pubk, msg);
    let deciph: u32 = decrypt(keys, cipher);
    println!("\nPrivate p: {}, q: {}", keys.0, keys.1);
    println!("Public p*q: {}", pubk);
    println!(
        "Original:  {}, Encrypted: {}\nDecrypted: {}\n",
        msg, cipher, deciph
    );
}
