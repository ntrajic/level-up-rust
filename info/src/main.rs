//fn info<T>(a: &T) {
fn info<T: std::fmt::Display>(a: &T) {
    //todo!();
    //println!("{}", a);
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    // use std::ffi::CString;
    
    // let c = CString::new("?").unwrap();
    // info(&input);

    // Advanced 2
    // use std::path::Path;
    // let d = Path::new("/tmp/linkedin-learning");
    // info(d);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }


// $ make test
// cargo test --quiet
// error[E0412]: cannot find type `T` in this scope
//  --> src/main.rs:1:13
//   |
// 1 | fn info(a: &T) {
//   |             ^ not found in this scope
//   |
// help: you might be missing a type parameter
//   |
// 1 | fn info<T>(a: &T) {
//   |        +++

// For more information about this error, try `rustc --explain E0412`.
// error: could not compile `info` (bin "info" test) due to previous error
// make: *** [Makefile:16: test] Error 101

// $ make build
// cargo build
// Compiling info v0.1.0 (/workspaces/level-up-rust/info)
// warning: unused variable: `a`
// --> src/main.rs:2:31
// |
// 2 | fn info<T: std::fmt::Display>(a: &T) {
// |                               ^ help: if this is intentional, prefix it with an underscore: `_a`
// |
// = note: `#[warn(unused_variables)]` on by default

// warning: `info` (bin "info") generated 1 warning (run `cargo fix --bin "info"` to apply 1 suggestion)
//  Finished dev [unoptimized + debuginfo] target(s) in 0.22s
// $ 



// $ make build
// cargo build
// Compiling info v0.1.0 (/workspaces/level-up-rust/info)
// warning: unused variable: `a`
// --> src/main.rs:2:31
// |
// 2 | fn info<T: std::fmt::Display>(a: &T) {
// |                               ^ help: if this is intentional, prefix it with an underscore: `_a`
// |
// = note: `#[warn(unused_variables)]` on by default

// warning: `info` (bin "info") generated 1 warning (run `cargo fix --bin "info"` to apply 1 suggestion)
//  Finished dev [unoptimized + debuginfo] target(s) in 0.22s
// $ 