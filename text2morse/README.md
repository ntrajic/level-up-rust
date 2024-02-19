Level Up Rust: Challenge 5
Your challenge is to implement a trait, MorseCode for String. The MorseCode trait contains a method to_morse_code() that returns a Message.

The Message type is defined within the sample code. It is a type alias for Vec<Letter>, where Letter represents a single character from the Morse code alphabet (A-Z, 0-9).

As a refresher on Morse code, individual characters are made up of short bursts of "pulses", which can either be short (.) or long (_).

Here are a few characters from the alphabet and their Morse code equivalents:

A	 _. 
B	_... 
C	 _._. 
...
X	 _.._ 
Y	 _.__ 
Z	 __.. 
To represent pulses in Rust, we'll use an enum:

enum Pulse {
    Short,
    Long,
}
Each character in the alphabet (A-Z, 0-9) takes up a variable number of pulses to be represented. Letters that occur more frequently in English, such as E and A, take fewer letters to be represented. To accomodate this, we'll represent a Letter as a Vec<Pulse>.

type Letter = Vec<Pulse>;
With Letter defined, we're able to build messages. A Message is defined as:

type Message = Vec<Letter>;
It is a type alias for the `Vec<

Testing your code
To test your solution, use cargo test.

$ cargo test
...