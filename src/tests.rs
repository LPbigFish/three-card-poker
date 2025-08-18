use std::{cmp::Ordering, time};

use itertools::Itertools;
use rand::{SeedableRng, rngs::StdRng};

use crate::components::{
    card::{Card, CardValue, SUIT},
    deck::Deck,
    hand::{Hand, HandValue},
    strategy::{Action, Strategy},
};

use CardValue::*;
use SUIT::*;

#[test]
fn compare_cards() {
    // Test card 1
    let c1 = Card::new(TWO, HEARTS);
    // Test card 2
    let c2 = Card::new(ACE, DIAMONDS);
    // Test card 3
    let c3 = Card::new(FIVE, CLUBS);
    // Test card 4
    let c4 = Card::new(FIVE, SPADES);

    assert_eq!(c1 < c2, true);
    assert_eq!(c3 == c4, true);
    assert_eq!(c2 < c4, false);
}

#[test]
fn test_hand_value() {
    let straight = Hand::new([
        Card::new(ACE, HEARTS),
        Card::new(KING, SPADES),
        Card::new(QUEEN, SPADES),
    ]);

    let royal_flush = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(KING, SPADES),
        Card::new(QUEEN, SPADES),
    ]);

    let three_aces = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(ACE, DIAMONDS),
        Card::new(ACE, HEARTS),
    ]);

    let pair_ace_five = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(ACE, DIAMONDS),
        Card::new(FIVE, HEARTS),
    ]);

    let straight_flush = Hand::new([
        Card::new(JACK, SPADES),
        Card::new(NINE, SPADES),
        Card::new(TEN, SPADES),
    ]);

    let ace_king_flush = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(KING, SPADES),
        Card::new(FIVE, SPADES),
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
        Card::new(ACE, HEARTS),
        Card::new(KING, SPADES),
        Card::new(QUEEN, SPADES),
    ]);

    let royal_flush = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(KING, SPADES),
        Card::new(QUEEN, SPADES),
    ]);

    let three_aces = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(ACE, DIAMONDS),
        Card::new(ACE, HEARTS),
    ]);

    let pair_ace_five = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(ACE, DIAMONDS),
        Card::new(FIVE, HEARTS),
    ]);

    let straight_flush = Hand::new([
        Card::new(JACK, SPADES),
        Card::new(NINE, SPADES),
        Card::new(TEN, SPADES),
    ]);

    let ace_king_flush = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(KING, SPADES),
        Card::new(FIVE, SPADES),
    ]);

    let three_tens = Hand::new([
        Card::new(TEN, SPADES),
        Card::new(TEN, DIAMONDS),
        Card::new(TEN, HEARTS),
    ]);

    let pair_ace_four = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(ACE, DIAMONDS),
        Card::new(FOUR, HEARTS),
    ]);

    let ace_king_flush_lower = Hand::new([
        Card::new(ACE, SPADES),
        Card::new(KING, SPADES),
        Card::new(FOUR, SPADES),
    ]);

    assert!(straight > pair_ace_five);
    assert!(royal_flush > straight_flush);
    assert!(three_aces > three_tens);
    assert!(pair_ace_five > pair_ace_four);
    assert!(ace_king_flush > ace_king_flush_lower)
}

#[test]
fn generate_decks() {
    let mut rng = StdRng::seed_from_u64(65u64);
    let beg = time::Instant::now();
    let mut decks = (0..1_000_000_000).map(|_| Deck::new(&mut rng));
    println!("{:?}", beg.elapsed());
    let (p, d) = decks.next().unwrap_or_default().deal_both();
    println!("{} vs {}", p, d);
    println!(
        "{} wins!",
        match p.cmp(&d) {
            Ordering::Less => "Dealer",
            Ordering::Equal => "Nobody",
            Ordering::Greater => "Player",
        }
    )
}

#[test]
fn test_strategy() {
    let strat = Strategy::default();
    let mut rng = StdRng::seed_from_u64(65u64);
    let mut decks = (0..1_000_000).map(|_| Deck::new(&mut rng));

    let game_result1 = decks.next().unwrap_or_default();
    println!(
        "Game result: {}\n",
        game_result1.get_game_result(Some(&strat), Some(Action::None))
    );
    let game_result2 = decks.next().unwrap_or_default();
    println!(
        "Next game result: {}",
        game_result2.get_game_result(
            Some(&strat),
            Some(game_result1.get_game_result(None, None).next_action())
        )
    );
}

#[test]
fn test_strategy_with_balance() {
    let strat = Strategy::default();
    let mut rng = StdRng::seed_from_u64(65u64);
    let decks = (0..1_000_000).map(|_| Deck::new(&mut rng));
    let res = decks.tuple_windows().map(|(a, b)| {
        let game_result1 = a.get_game_result(Some(&strat), Some(Action::None));
        let game_result2 = b.get_game_result(Some(&strat), Some(game_result1.next_action()));

        (
            (game_result1.ante_bet() * if game_result1.played() { 2f32 } else { 1f32 }) + (game_result2.ante_bet() * if game_result2.played() { 2f32 } else { 1f32 }),
            game_result1.outcome() + game_result2.outcome(),
        )
    });

    let (total_ante, total_outcome) = res
        .take(10_000)
        .fold((0f32, 0f32), |(cante, cout), (x, y)| (cante + x, cout + y));

    println!(
        "Total ante: {}, Total outcome: {}",
        total_ante, total_outcome
    );
}
