mod vigenere {
    
    /*
    * The Vigenère cipher is a classic symmetric encryption algorithm that uses a keyword or phrase as the key. 
    * It is a form of polyalphabetic substitution cipher, meaning it uses multiple substitution alphabets. 
    * Here's how the Vigenère cipher works:
    *
    * Key Expansion: 
    * The first step is to expand the key to match the length of the plaintext. 
    * This is typically done by repeating the key until it matches the length of the plaintext. 
    * For example, if the plaintext is "HELLO" and the key is "KEY", the expanded key would be "KEYKE".
    *
    * Encryption: 
    * Each letter of the plaintext is encrypted using a different Caesar cipher based on a letter of the key.
    *
    * To encrypt a character, find the corresponding letter in the key.
    * Treat this letter as the shift value in a Caesar cipher.
    * Shift the plaintext character by the value of the key character.
    * If the result goes beyond 'Z' (for uppercase) or 'z' (for lowercase), wrap around to 'A' or 'a' respectively.
    */
    pub fn encrypt(plaintext: &str, key: &str) -> String {
        String::new()
    }


   /*
    * Decryption: 
    * Decryption is essentially the reverse process of encryption. 
    * Each letter of the ciphertext is decrypted using the corresponding letter of the key.
    *
    * To decrypt a character, find the corresponding letter in the key.
    * Treat this letter as the reverse shift value in a Caesar cipher (i.e., shift backward).
    * Shift the ciphertext character by the negative value of the key character.
    * If the result goes beyond 'Z' (for uppercase) or 'z' (for lowercase), wrap around to 'A' or 'a' respectively.
    * The security of the Vigenère cipher comes from the use of multiple Caesar ciphers with different shift values 
    * based on the key. This makes frequency analysis and other traditional cryptanalysis methods less effective 
    * compared to simpler substitution ciphers like the Caesar cipher. 
    * However, the Vigenère cipher is still vulnerable to some attacks, particularly if the key is short or 
    * if patterns exist in the plaintext.
    *
    * Despite its vulnerability to certain attacks, the Vigenère cipher was considered unbreakable for centuries and 
    * was extensively used until more secure cryptographic techniques were developed. 
    * It remains a fundamental encryption algorithm in the history of cryptography.
    */
    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        todo!()
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}


// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/runlengthencoding$ make test
// cargo test --quiet

// running 3 tests
// ...
// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/runlengthencoding$ cd ../vigenerecipher/
// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/vigenerecipher$ make test
// cargo test --quiet
// warning: unused variable: `plaintext`
//   --> src/main.rs:21:20
//    |
// 21 |     pub fn encrypt(plaintext: &str, key: &str) -> String {
//    |                    ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_plaintext`
//    |
//    = note: `#[warn(unused_variables)]` on by default

// warning: unused variable: `key`
//   --> src/main.rs:21:37
//    |
// 21 |     pub fn encrypt(plaintext: &str, key: &str) -> String {
//    |                                     ^^^ help: if this is intentional, prefix it with an underscore: `_key`

// warning: unused variable: `ciphertext`
//   --> src/main.rs:45:20
//    |
// 45 |     pub fn decrypt(ciphertext: &str, key: &str) -> String {
//    |                    ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_ciphertext`

// warning: unused variable: `key`
//   --> src/main.rs:45:38
//    |
// 45 |     pub fn decrypt(ciphertext: &str, key: &str) -> String {
//    |                                      ^^^ help: if this is intentional, prefix it with an underscore: `_key`

// warning: function `encrypt` is never used
//   --> src/main.rs:21:12
//    |
// 21 |     pub fn encrypt(plaintext: &str, key: &str) -> String {
//    |            ^^^^^^^
//    |
//    = note: `#[warn(dead_code)]` on by default


// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

