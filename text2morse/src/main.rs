#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

///
/// implement a trait, MorseCode for String. The MorseCode trait contains a method to_morse_code() that returns a Message.
///
/// The Message type is defined within the sample code. It is a type alias for Vec<Letter>, where Letter represents a single character from the Morse code alphabet (A-Z, 0-9).
///
/// As a refresher on Morse code, individual characters are made up of short bursts of "pulses", which can either be short (.) or long (_).
///
/// Here are a few characters from the alphabet and their Morse code equivalents:
///
/// A	 _. 
/// B	_... 
/// C	 _._. 
/// ...
/// X	 _.._ 
/// Y	 _.__ 
/// Z	 __.. 
/// To represent pulses in Rust, we'll use an enum:
///
/// enum Pulse {
///     Short,
///     Long,
/// }
/// Each character in the alphabet (A-Z, 0-9) takes up a variable number of pulses to be represented. Letters that occur more frequently in English, such as E and A, take fewer letters to be represented. To accomodate this, we'll represent a Letter as a Vec<Pulse>.
///
/// type Letter = Vec<Pulse>;
/// With Letter defined, we're able to build messages. A Message is defined as:
///
/// type Message = Vec<Letter>;
/// It is a type alias for the `Vec<Letter>`

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut msg = Vec::with_capacity(self.len());
        for c in self.chars() {
            let letter = match c {
                'A' | 'a' => vec![Short, Long],
                'B' | 'b' => vec![Long, Short, Short, Short],
                'C' | 'c' => vec![Long, Short, Long, Short],
                'D' | 'd' => vec![Long, Short, Short],
                'E' | 'e' => vec![Short],
                'F' | 'f' => vec![Short, Short, Long, Short],
                'G' | 'g' => vec![Long, Long, Short],
                'H' | 'h' => vec![Short, Short, Short, Short, Short],
                'I' | 'i' => vec![Short, Short],
                'J' | 'j' => vec![Short, Long, Long, Long],
                'K' | 'k' => vec![Long, Short, Long],
                'L' | 'l' => vec![Short, Long, Short, Short],
                'M' | 'm' => vec![Long, Long],
                'N' | 'n' => vec![Long, Short],
                'O' | 'o' => vec![Long, Long, Long],
                'P' | 'p' => vec![Short, Long, Long, Short],
                'Q' | 'q' => vec![Long, Long, Short, Long],
                'R' | 'r' => vec![Short, Long, Short],
                'S' | 's' => vec![Short, Short, Short],
                'T' | 't' => vec![Long],
                'U' | 'u' => vec![Short, Short, Long],
                'V' | 'v' => vec![Short, Short, Short, Long],
                'W' | 'w' => vec![Short, Long, Long],
                'X' | 'x' => vec![Long, Short, Short, Long],
                'Y' | 'y' => vec![Long, Short, Long, Long],
                'Z' | 'z' => vec![Long, Long, Short, Short],

                '1' => vec![Short, Long, Long, Long, Long],
                '2' => vec![Short, Short, Long, Long, Long],
                '3' => vec![Short, Short, Short, Long, Long],
                '4' => vec![Short, Short, Short, Short, Long],
                '5' => vec![Short, Short, Short, Short, Short],
                '6' => vec![Long, Short, Short, Short, Short],
                '7' => vec![Long, Long, Short, Short, Short],
                '8' => vec![Long, Long, Long, Short, Short],
                '9' => vec![Long, Long, Long, Long, Short],
                '0' => vec![Long, Long, Long, Long, Long],
                _ => continue,
            };

            msg.push(letter);
        }
        msg
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}


//  *  Executing task: cargo test --package text2morse --bin text2morse --  --nocapture 

// Compiling text2morse v0.1.0 (/mnt/c/SRC/Rust/level-up-rust/text2morse)
// Finished test [unoptimized + debuginfo] target(s) in 0.83s
// Running unittests src/main.rs (target/debug/deps/text2morse-33ce056cb2c1ba7b)

// running 2 tests
// test hello_world ... ok
// test whole_alphabet ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s