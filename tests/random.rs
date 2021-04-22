use primes::*;
use toy_rsa::*;

///let pubk: u64 = (keys.0 as u64) * (keys.1 as u64);
///let msg: u32 = rand::thread_rng().gen::<u32>();
///let cipher: u64 = encrypt(pubk, msg);
///let deciph: u32 = decrypt(keys, cipher);

#[test]
fn keys_test() {
    let keys = genkey();
    assert_eq!(is_prime(keys.0 as u64), true);
    assert_eq!(is_prime(keys.0 as u64), true);
    assert!((keys.0 as u64 * keys.1 as u64) < u64::max_value());
}

#[test]
fn msg_size() {
    let keys = genkey();
    let pubk: u64 = keys.0 as u64 * keys.1 as u64;
    let msg1: u32 = u32::max_value();
    let msg2: u32 = u32::min_value();
    let c1: u64 = encrypt(pubk, msg1);
    let c2: u64 = encrypt(pubk, msg2);
    assert_eq!(msg1, decrypt(keys, c1));
    assert_eq!(msg2, decrypt(keys, c2));
}

#[test]
#[should_panic]
fn bad_public() {
    let keys = genkey();
    let pubk: u64 = (keys.0 as u64 * keys.1 as u64) - 1;
    let msg1: u32 = u32::max_value();
    let c1: u64 = encrypt(pubk, msg1);
    assert_eq!(msg1, decrypt(keys, c1));
}

#[test]
#[should_panic]
fn bad_private_p() {
    let keys = genkey();
    let pubk: u64 = keys.0 as u64 * keys.1 as u64;
    let msg1: u32 = u32::max_value();
    let c1: u64 = encrypt(pubk, msg1);
    assert_eq!(msg1, decrypt((keys.0 - 1, keys.1), c1));
}

#[test]
#[should_panic]
fn bad_private_q() {
    let keys = genkey();
    let pubk: u64 = keys.0 as u64 * keys.1 as u64;
    let msg1: u32 = u32::max_value();
    let c1: u64 = encrypt(pubk, msg1);
    assert_eq!(msg1, decrypt((keys.0, keys.1 - 1), c1));
}
