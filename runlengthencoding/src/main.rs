// Python implementation of Run-Length-Encoding
// ######################################################################################################
// # Implement RUN-LENGTH ENCODING
// ######################################################################################################
// # "aaaabcccaa" rle_encode -> "4a1b3c2a".
// # "4a1b3c2a"   rle_decode -> "aaaabcccaa"
// ######################################################################################################
//
// from typing import List
//
// # "aaaabcccaa" rle_encode -> "4a1b3c2a".
// # => compare i and i-1, for i=1..len(s); new char found or we reqched end of s: s[i-1] != s[i] or i==len(s)
// #  rle_encode('b4a'):   'b' != '4' => write 1b
// def rle_encode(s:str)->str:
//
//     result: List[chr] = []
//     count: int = 1                                        # <------ count init to 1 not to 0
//
//     for i in range (1, len(s) + 1):                       # <----- i from [1, len(s)+1),    len(s)]
//         if i == len(s) or s[i-1] != s[i]:                 # <--- cannot invert or expression members
//             result.append (str(count) + s[i-1])
//             count = 1                                     # <----- reset to 1: 1st time we detect diff in neighboring chars, set count to 1 not to 0 !!!
//         else: # (not end of string s) and s[i-1] == s[i]  # de morgan
//             count += 1
//         #
//     #
//     return ''.join(result)
// #
//
// #15b; cnt=15:  15 = 1 * 10 exp(1) + 5 * 10 exp(0)
// # 4a1b3c2a"   rle_decode -> "aaaabcccaa"
// def rle_decode(s:str)->str:
//
//     count:int = 0
//     result: List[chr] = []
//     c: chr
//
//     for c in s:
//         if c.isdigit():
//             count = count * 10 + int(c)           # num = num * 10 + num_val_digit;   num_val_digit_in_C = c - '0'; in py num_val_digit_Py = int(c), if isdigit(c)==True
//         else:
//             result.append(count*c)                # in C: result.append(count, c)   ie. count times char c append
//             count = 0                             # must reset, for another count
//         #
//     #
//     return ''.join(result)
// #
//
// if __name__ == '__main__':
//     ...
//     orig = "aaaabcccaa"
//     encoded = "4a1b3c2a"
//     print("rle_encode('aaaabcccaa') is '4a1b3c2a' ", rle_encode(orig) )      # rle_encode('aaaabcccaa') is '4a1b3c2a'
//     print("rle_decode('4a1b3c2a') is 'aaaabcccaa' ", rle_decode(encoded) )   # rle_decode('4a1b3c2a') is 'aaaabcccaa'

/// Rust RLE implementation - End of impl.

mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut count = 0;
        let mut prev: Option<char> = None;
        let mut encoded = String::with_capacity(text.len() / 2);
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            if prev.is_none() {
                prev = Some(c);
            }

            if prev.unwrap() != c || count == 9 {
                encoded.push_str(&format!("{}{}", count, prev.unwrap()));
                count = 0
            }
            prev = Some(c);
            count += 1;
        }

        // protect against empty string
        if let Some(prev) = prev {
            encoded.push_str(&format!("{}{}", count, prev));
        }
        encoded
    }

    pub fn decode(text: &str) -> String {
        let mut decoded = String::with_capacity(text.len() * 2);
        let mut chars = text.chars();

        while let (Some(n), Some(c)) = (chars.next(), chars.next()) {
            let n = n.to_digit(10).unwrap();
            for _ in 0..n {
                decoded.push(c);
            }
        }

        decoded
    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}

// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/runlengthencoding$ make test
// cargo test --quiet
//
// running 3 tests
// ...
// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
