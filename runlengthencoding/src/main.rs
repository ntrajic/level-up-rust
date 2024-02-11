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

/// Rust RLE implementation - Beginning

mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        todo!()
    }
    
    pub fn decode(text: &str) -> String {
        todo!()
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
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
