use std::{cmp::Ordering, time};

use rand::rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::components::{card::{Card, CardValue, SUIT}, deck::Deck, hand::{Hand, HandValue}};

#[test]
fn compare_cards() {
    // Test card 1
    let c1 = Card::new(2, SUIT::HEARTS);
    // Test card 2
    let c2 = Card::new(14, SUIT::DIAMONDS);
    // Test card 3
    let c3 = Card::new(5, SUIT::CLUBS);
    // Test card 4
    let c4 = Card::new(5, SUIT::SPADES);

    assert_eq!(c1 < c2, true);
    assert_eq!(c3 == c4, true);
    assert_eq!(c2 < c4, false);
}

#[test]
fn test_hand_value() {
    let straight = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::HEARTS), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::QUEEN as u8, SUIT::SPADES) 
    ]);

    let royal_flush = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::QUEEN as u8, SUIT::SPADES) 
    ]);

    let three_aces = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::ACE as u8, SUIT::DIAMONDS),
        Card::new(CardValue::ACE as u8, SUIT::HEARTS) 
    ]);

    let pair_ace_five = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::ACE as u8, SUIT::DIAMONDS),
        Card::new(CardValue::FIVE as u8, SUIT::HEARTS) 
    ]);

    let straight_flush = Hand::new([ 
        Card::new(CardValue::JACK as u8, SUIT::SPADES), 
        Card::new(CardValue::NINE as u8, SUIT::SPADES),
        Card::new(CardValue::TEN as u8, SUIT::SPADES) 
    ]);

    let ace_king_flush = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::FIVE as u8, SUIT::SPADES) 
    ]);

    assert_eq!(straight.get_hand_value(), HandValue::Straight);
    
    assert_eq!(royal_flush.get_hand_value(), HandValue::RoyalFlush);
    
    assert_eq!(three_aces.get_hand_value(), HandValue::ThreeOfAKind);

    assert_eq!(pair_ace_five.get_hand_value(), HandValue::Pair);

    assert_eq!(straight_flush.get_hand_value(), HandValue::StraightFlush);

    assert_eq!(ace_king_flush.get_hand_value(), HandValue::Flush);
}

#[test]
fn hand_comparison() {
        let straight = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::HEARTS), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::QUEEN as u8, SUIT::SPADES) 
    ]);

    let royal_flush = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::QUEEN as u8, SUIT::SPADES) 
    ]);

    let three_aces = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::ACE as u8, SUIT::DIAMONDS),
        Card::new(CardValue::ACE as u8, SUIT::HEARTS) 
    ]);

    let pair_ace_five = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::ACE as u8, SUIT::DIAMONDS),
        Card::new(CardValue::FIVE as u8, SUIT::HEARTS) 
    ]);

    let straight_flush = Hand::new([ 
        Card::new(CardValue::JACK as u8, SUIT::SPADES), 
        Card::new(CardValue::NINE as u8, SUIT::SPADES),
        Card::new(CardValue::TEN as u8, SUIT::SPADES) 
    ]);

    let ace_king_flush = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::FIVE as u8, SUIT::SPADES) 
    ]);

    let three_tens = Hand::new([ 
        Card::new(CardValue::TEN as u8, SUIT::SPADES), 
        Card::new(CardValue::TEN as u8, SUIT::DIAMONDS),
        Card::new(CardValue::TEN as u8, SUIT::HEARTS) 
    ]);

    let pair_ace_four = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::ACE as u8, SUIT::DIAMONDS),
        Card::new(CardValue::FOUR as u8, SUIT::HEARTS) 
    ]);

    let ace_king_flush_lower = Hand::new([ 
        Card::new(CardValue::ACE as u8, SUIT::SPADES), 
        Card::new(CardValue::KING as u8, SUIT::SPADES),
        Card::new(CardValue::FOUR as u8, SUIT::SPADES) 
    ]);

    assert!(straight > pair_ace_five);
    assert!(royal_flush > straight_flush);
    assert!(three_aces > three_tens);
    assert!(pair_ace_five > pair_ace_four);
    assert!(ace_king_flush > ace_king_flush_lower)
}

#[test]
fn generate_decks() {
    let mut rng = rng();
    let beg = time::Instant::now();
    let mut decks = (0..1_000_000_000).map(|_| Deck::new(&mut rng));
    let (p, d) = decks.next().unwrap_or_default().deal_both();
    println!("{:?}", beg.elapsed());
    println!("{} vs {}", p, d);
    println!("{} wins!", match p.cmp(&d) {
        Ordering::Less => "Dealer",
        Ordering::Equal => "Nobody",
        Ordering::Greater => "Player",
    })
}