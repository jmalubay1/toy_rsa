###Jordan Malubay
##Toy-RSA

Toy-rsa is a Rust library crate that implements a simple RSA key encryption and decryption.  



###Usage
```rust
use toy_rsa::*;

genkey() -> (u32, u32)

encrypt(public: u64, msg: u32) -> u64

decrypt(keys: (u32, u32), msg: u64) -> u32
```
genkey() - Generates two private prime keys in a tuple (p, q).

encrypt() - Uses the public key (p * q) to encrypt an unsigned 32 bit integer into a 64 bit unsigned integer.

decrypt() - Uses the private key tuple (p, q) and the encrypted integer to retrieve the original 32 bit unsigned integer. 

###Implementation Notes
Implementing the functions for the RSA encryption and decryption was pretty straight forward.  My biggest issues with this project was implementing the documentation. I had a hard time checking the documentation using putty and checking the builds over the network.