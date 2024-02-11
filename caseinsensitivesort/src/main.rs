fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    //todo!();
    usernames.sort_by(|x, y| {x.as_ref().to_lowercase().cmp(&y.as_ref().to_lowercase())});
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}

// Compiling caseinsensitivesort v0.1.0 (/mnt/c/SRC/Rust/level-up-rust/caseinsensitivesort)
// Finished test [unoptimized + debuginfo] target(s) in 1.45s
//  Running unittests src/main.rs (target/debug/deps/caseinsensitivesort-4e0a6fa3a40f97b1)

// running 1 test
// test five_users ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/caseinsensitivesort$ make run
// cargo run                               
//    Compiling caseinsensitivesort v0.1.0 (/mnt/c/SRC/Rust/level-up-rust/caseinsensitivesort)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.78s
//      Running `target/debug/caseinsensitivesort`
// unsorted: ["Todd", "Amy", "mike99", "Jennifer", "alison"]
// sorted:   ["alison", "Amy", "Jennifer", "mike99", "Todd"] 


// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/caseinsensitivesort$ make test
// cargo test --quiet
// error[E0428]: the name `sort_usernames` is defined multiple times
//   --> src/main.rs:20:2
//    |
// 1  | fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
//    | -------------------------------------------------------- previous definition of the value `sort_usernames` here
// ...
// 20 | }fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `sort_usernames` redefined here
//    |
//    = note: `sort_usernames` must be defined only once in the value namespace of this module

// error[E0428]: the name `main` is defined multiple times
//   --> src/main.rs:24:1
//    |
// 5  | fn main() {
//    | --------- previous definition of the value `main` here
// ...
// 24 | fn main() {
//    | ^^^^^^^^^ `main` redefined here
//    |
//    = note: `main` must be defined only once in the value namespace of this module

// error[E0428]: the name `five_users` is defined multiple times
//   --> src/main.rs:33:1
//    |
// 14 | fn five_users() {
//    | --------------- previous definition of the value `five_users` here
// ...
// 33 | fn five_users() {
//    | ^^^^^^^^^^^^^^^ `five_users` redefined here
//    |
//    = note: `five_users` must be defined only once in the value namespace of this module

// For more information about this error, try `rustc --explain E0428`.
// error: could not compile `caseinsensitivesort` (bin "caseinsensitivesort" test) due to 3 previous errors
// make: *** [Makefile:16: test] Error 101ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/caseinsensitivesort$ make test
// cargo test --quiet
// error[E0428]: the name `sort_usernames` is defined multiple times
//   --> src/main.rs:20:2
//    |
// 1  | fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
//    | -------------------------------------------------------- previous definition of the value `sort_usernames` here
// ...
// 20 | }fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `sort_usernames` redefined here
//    |
//    = note: `sort_usernames` must be defined only once in the value namespace of this module

// error[E0428]: the name `main` is defined multiple times
//   --> src/main.rs:24:1
//    |
// 5  | fn main() {
//    | --------- previous definition of the value `main` here
// ...
// 24 | fn main() {
//    | ^^^^^^^^^ `main` redefined here
//    |
//    = note: `main` must be defined only once in the value namespace of this module

// error[E0428]: the name `five_users` is defined multiple times
//   --> src/main.rs:33:1
//    |
// 14 | fn five_users() {
//    | --------------- previous definition of the value `five_users` here
// ...
// 33 | fn five_users() {
//    | ^^^^^^^^^^^^^^^ `five_users` redefined here
//    |
//    = note: `five_users` must be defined only once in the value namespace of this module

// For more information about this error, try `rustc --explain E0428`.
// error: could not compile `caseinsensitivesort` (bin "caseinsensitivesort" test) due to 3 previous errors
// make: *** [Makefile:16: test] Error 101
