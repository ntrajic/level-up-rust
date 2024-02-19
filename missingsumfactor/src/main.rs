// Missing values in rust are an Option type:
//
// enum Option<T<> {
//    Some(T),
//    None,                 // variant None indictes a missing value
//}
// e.g 1                                          e.g.2:
// let data = vec![Some(4), None, Some(1), ];     let data = vec![None, None, None,];   
// let answer = sum_with_missing(data);           let anslwer = sum_with_missing(data);
// assert_eq!(answer, 5);                         assert_eq!(answer, 0)

fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    todo!();
}

fn main() {
    println!("");
}


#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}




// $ cargo test
//    Compiling removedups v0.1.0 (/workspaces/level-up-rust/missingsumfactor)
// warning: unused variable: `numbers`
//  --> src/main.rs:1:21
//   |
// 1 | fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
//   |                     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_numbers`
//   |
//   = note: `#[warn(unused_variables)]` on by default
//
// warning: `removedups` (bin "removedups" test) generated 1 warning (run `cargo fix --bin "removedups" --tests` to apply 1 suggestion)
//     Finished test [unoptimized + debuginfo] target(s) in 1.55s
//      Running unittests src/main.rs (target/debug/deps/removedups-af32a8e72402fd66)
//
// running 4 tests
// test all_missing ... FAILED
// test empty ... FAILED
// test no_missing ... FAILED
// test some_missing ... FAILED
//
// failures:
//
// ---- all_missing stdout ----
// thread 'all_missing' panicked at src/main.rs:2:5:
// not yet implemented
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// ---- empty stdout ----
// thread 'empty' panicked at src/main.rs:2:5:
// not yet implemented
//
// ---- no_missing stdout ----
// thread 'no_missing' panicked at src/main.rs:2:5:
// not yet implemented
//
// ---- some_missing stdout ----
// thread 'some_missing' panicked at src/main.rs:2:5:
// not yet implemented
//
//
// failures:
//     all_missing
//     empty
//     no_missing
//     some_missing
//
// test result: FAILED. 0 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass `--bin removedups`