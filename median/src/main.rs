fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }
    //Some(0.0)  // return Some(0.0)

    // in this example, a.partial_cmp(b).unwrap() is used
    // because f32 does not implement the Ord trait due to the fact that NaN (Not a Number)
    // is not comparable.
    // The partial_cmp method returns an Option<Ordering> instead of Ordering, and
    // unwrap() is used to get the Ordering value.
    //
    // a.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // println!("{:?}", a);

    a.sort_by(|x: &f32, y: &f32| x.partial_cmp(y).unwrap());

    let n_elements: usize = a.len();
    let middle: usize = n_elements / 2;
    let a_is_even: bool = n_elements % 2 == 0;

    let med: f32 = if a_is_even {
        (a[middle] + a[middle - 1]) / 2.0 // return average of two adjucent center elements
    } else {
        a[middle] // return middle one
    };

    Some(med) // return med packaged in Some()
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

// test driven:
// make test, until all tests pass
//
// $ make all <enter>
// OUT:
// cargo fmt --quiet
// cargo clippy --quiet
// cargo test --quiet
//
// running 4 tests
// ....
// test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// cargo build
//    Compiling median v0.1.0 (/workspaces/level-up-rust/median)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.20s
