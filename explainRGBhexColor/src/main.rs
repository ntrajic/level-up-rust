use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ParseColorError {
    RedOutOfBounds,
    BlueOutOfBounds,
    GreenOutOfBounds,
    Invalid,
}

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

impl FromStr for Rgb {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .or_else(|_| Err(ParseColorError::RedOutOfBounds))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .or_else(|_| Err(ParseColorError::BlueOutOfBounds))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .or_else(|_| Err(ParseColorError::GreenOutOfBounds))?;

            Ok(Rgb { r, g, b })
        } else {
            // no leading #
            Err(ParseColorError::Invalid)
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    // 
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}


// $ make test
// cargo test --quiet
// error[E0046]: not all trait items implemented, missing: `r`, `g`, `b`
//   --> src/main.rs:15:1
//    |
// 8  |     fn r(&self) -> u8;
//    |     ------------------ `r` from trait
// 9  |
// 10 |     fn g(&self) -> u8;
//    |     ------------------ `g` from trait
// 11 |
// 12 |     fn b(&self) -> u8;
//    |     ------------------ `b` from trait
// ...
// 15 | impl RgbChannels for Rgb {
//    | ^^^^^^^^^^^^^^^^^^^^^^^^ missing `r`, `g`, `b` in implementation
//
// error[E0046]: not all trait items implemented, missing: `Err`, `from_str`
//   --> src/main.rs:19:1
//    |
// 19 | impl FromStr for Rgb {
//    | ^^^^^^^^^^^^^^^^^^^^ missing `Err`, `from_str` in implementation
//    |
//    = help: implement the missing item: `type Err = /* Type */;`
//    = help: implement the missing item: `fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> { todo!() }`
//
// For more information about this error, try `rustc --explain E0046`.
// error: could not compile `explainRGBhexColor` (bin "explainRGBhexColor" test) due to 2 previous errors
// make: *** [Makefile:16: test] Error 101