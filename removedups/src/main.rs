fn unique(a: Vec<i32>) -> Vec<i32> {
    todo!()
}

// advanced 1: use generic types
// fn unique(a: Vec<T>) -> Vec<T> {
//     todo!();
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}


// $ make test   // or: cargo test
// OUTPUT
// cargo test --quiet
// warning: unused variable: `a`
//  --> src/main.rs:1:11
//   |
// 1 | fn unique(a: Vec<i32>) -> Vec<i32> {
//   |           ^ help: if this is intentional, prefix it with an underscore: `_a`
//   |
//   = note: `#[warn(unused_variables)]` on by default
//
//
// running 5 tests
// FFFFF
// failures:
//
// ---- sorted_list stdout ----
// thread 'sorted_list' panicked at src/main.rs:2:5:
// not yet implemented
//
// ---- empty_list stdout ----
// thread 'empty_list' panicked at src/main.rs:2:5:
// not yet implemented
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// ---- unsorted_list stdout ----
// thread 'unsorted_list' panicked at src/main.rs:2:5:
// not yet implemented
//
// ---- sorted_list_with_duplicates stdout ----
// thread 'sorted_list_with_duplicates' panicked at src/main.rs:2:5:
// not yet implemented
//
// ---- unsorted_list_with_duplicates stdout ----
// thread 'unsorted_list_with_duplicates' panicked at src/main.rs:2:5:
// not yet implemented
//
//
// failures:
//     empty_list
//     sorted_list
//     sorted_list_with_duplicates
//     unsorted_list
//     unsorted_list_with_duplicates
//
// test result: FAILED. 0 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass `--bin removedups`
// make: *** [Makefile:16: test] Error 101