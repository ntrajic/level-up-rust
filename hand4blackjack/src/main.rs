#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        use Card::*;

        let mut subtotal = 0;
        let mut aces_seen = 0;

        for card in self.cards.iter() {
            subtotal += match card {
                Ace => {
                    aces_seen += 1;
                    0
                },
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Jack => 10,
                Queen => 10,
                King => 10,
            };
        }

        for _ in 0..aces_seen {
            let ace_value = if subtotal <= 10 { 11 } else { 1 };
            subtotal += ace_value;
        }

        subtotal
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}

// ntrajic@DESKTOP-6PK7L32:/mnt/c/SRC/Rust/level-up-rust/hand4blackjack$ make test
// cargo test --quiet

// running 4 tests
// .FFF
// failures:

// ---- oops stdout ----
// thread 'oops' panicked at 'assertion failed: hand.is_loosing_hand()', src/main.rs:84:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ---- risky_hand stdout ----
// thread 'risky_hand' panicked at 'assertion failed: `(left == right)`
//   left: `0`,
//  right: `21`', src/main.rs:74:5

// ---- strong_hand stdout ----
// thread 'strong_hand' panicked at 'assertion failed: `(left == right)`
//   left: `0`,
//  right: `21`', src/main.rs:64:5

// failures:
//     oops
//     risky_hand
//     strong_hand

// test result: FAILED. 1 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
