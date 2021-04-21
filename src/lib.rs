///! RSA encryption library
///!
///! Jordan Malubay 2021
use std::convert::TryInto;
use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65537;

///λ(pq) returns the LCM of p-1 and q-1
pub fn lambda(p: u32, q: u32) -> u64 {
    lcm((p as u64) - 1, (q as u64) - 1)
}

/// Generate a pair of primes in the range `2**30..2**31`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    let mut p: u32;
    let mut q: u32;
    loop {
        p = rsa_prime();
        q = rsa_prime();
        let lam: u64 = lambda(p, q);
        if EXP < lam && gcd(EXP, lam) == 1 {
            break;
        }
    }
    (p, q)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d: u64 = modinverse(EXP, lambda(key.0, key.1));
    modexp(msg as u64, d, (key.0 * key.1) as u64)
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let key = genkey();
        let msg: u32 = 123456789;
        let cipher: u64 = encrypt(key.0, msg);
        let deciph: u32 = decrypt(key, cipher);
        assert_eq!(msg, deciph);
    }
}
