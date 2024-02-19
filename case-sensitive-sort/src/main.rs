fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    todo!();
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

// $ make test
// cargo test --quiet
// warning: unused variable: `usernames`
//  --> src/main.rs:1:34
//   |
// 1 | fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
//   |                                  ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_usernames`
//   |
//   = note: `#[warn(unused_variables)]` on by default


// running 1 test
// F
// failures:

// ---- five_users stdout ----
// thread 'five_users' panicked at src/main.rs:2:5:
// not yet implemented
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


// failures:
//     five_users

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass `--bin removedups`
// make: *** [Makefile:16: test] Error 101