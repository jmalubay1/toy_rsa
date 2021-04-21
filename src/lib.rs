///! RSA encryption library
///!
///! Jordan Malubay 2021

use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65537;

/// Generate a pair of primes in the range `2**30..2**31`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
  loop {
    let p: u32 = rsa_prime();
    let q: u32 = rsa_prime();
    let lam: u32 = lcm(p-1,q-1);
    if EXP < lam && gcd(EXP,lam) == 1 { break }
  }
  (p,q)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {

}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
