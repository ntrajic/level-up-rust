// convert from raw::String to Isbn::digits: Vec<u8>, for struct Isbn impl
// digits should not be .GT. 32,  too short, and checksum should be calculated/applied properly
// each digit = weight x digit_value@idx_in_digits,  weight is in a list: [0, 1, 2]
// 

use std::str::FromStr;


struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl Isbn {
    fn new(raw: String, digits: Vec<u8>) -> Self {
        Self { raw, digits }
    }
}

#[derive(Debug)]
enum InvalidIsbn {                          // describe errors enumInvalidIsbn structure
    TooLong,                                // len > 13
    TooShort,                               // len < 13
    InvalidCharacter(usize, char),          //  InvalidCharacter(usize, char);
    FailedChecksum,                         // for each 12 digits, calculate checksum
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];

        for (i, c) in s.char_indices() {            // iterate thru enumerated character tuples (idx, char) returned by string.char_indices()                  
            match c {                                           // match on two choices: 0. sign (negative) 1. true digit 2. InvalidIsbn struct, after unpacking/unwrapping the Option    
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => {
                    return Err(InvalidIsbn::InvalidCharacter(i, c));
                }
            }
        }
        match digits.len() {
            len if len < 13 => return Err(InvalidIsbn::TooShort),
            len if len > 13 => return Err(InvalidIsbn::TooLong),
            _ => (),
        }
        
        match digits.get(12) {
            Some(&digit) if digit != calculate_check_digit(&digits) => {
                return Err(InvalidIsbn::FailedChecksum);
            }
            _ => (),
        }
        
        Ok(Isbn {
            digits,
            raw: s.to_string(),
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];

    let weights_applied: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&x, &y)| x * y)
        .map(|x| x as u32)
        .sum();

    let check_digit = 10 - (weights_applied % 10);

    match check_digit {
        10 => 0_u8,
        m => m as u8,
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0"
        .parse()
        .unwrap();

    println!("Rust in Action's ISBN-13 ({}) is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}

// (bin "validateisbn") 
//     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//      Running `target/debug/validateisbn`
// Rust in Action's ISBN-13 (978-3-16-148410-0) is valid!
//  *  Terminal will be reused by tasks, press any key to close it. 


// $ make test
// cargo test --quiet
// warning: unused variable: `s`
//   --> src/main.rs:11:17
//    |
// 11 |     fn from_str(s: &str) -> Result<Self, Self::Err> {
//    |                 ^ help: if this is intentional, prefix it with an underscore: `_s`
//    |
//    = note: `#[warn(unused_variables)]` on by default
//
// warning: unused variable: `digits`
//   --> src/main.rs:23:26
//    |
// 23 | fn calculate_check_digit(digits: &[u8]) -> u8 {
//    |                          ^^^^^^ help: if this is intentional, prefix it with an underscore: `_digits`
//
// warning: field `digits` is never read
//  --> src/main.rs:5:5
//   |
// 3 | struct Isbn {
//   |        ---- field in this struct
// 4 |     raw: String,
// 5 |     digits: Vec<u8>,
//   |     ^^^^^^
//   |
//   = note: `#[warn(dead_code)]` on by default
//
//
// running 2 tests
// FF
// failures:
//
// ---- can_correctly_calculate_check_digits stdout ----
// thread 'can_correctly_calculate_check_digits' panicked at src/main.rs:24:5:
// not yet implemented
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// ---- rust_in_action stdout ----
// thread 'rust_in_action' panicked at src/main.rs:12:9:
// not yet implemented
//
//
// failures:
//     can_correctly_calculate_check_digits
//     rust_in_action
//
// test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass `--bin validatgeisbn`
// make: *** [Makefile:16: test] Error 101