use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb; // TODO: design data structure

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    // TODO: implement trait
}

impl FromStr for Rgb {
    // TODO: implement trait
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