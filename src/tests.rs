use crate::components::{card::{Card, CARD_VALUE, SUIT}, hand::{Hand, HAND_VALUE}};

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
    let hand1 = Hand::new([ 
        Card::new(CARD_VALUE::ACE as u8, SUIT::HEARTS), 
        Card::new(CARD_VALUE::KING as u8, SUIT::SPADES),
        Card::new(CARD_VALUE::QUEEN as u8, SUIT::SPADES) 
    ]);

    let hand2 = Hand::new([ 
        Card::new(CARD_VALUE::ACE as u8, SUIT::SPADES), 
        Card::new(CARD_VALUE::KING as u8, SUIT::SPADES),
        Card::new(CARD_VALUE::QUEEN as u8, SUIT::SPADES) 
    ]);

    let hand3 = Hand::new([ 
        Card::new(CARD_VALUE::ACE as u8, SUIT::SPADES), 
        Card::new(CARD_VALUE::ACE as u8, SUIT::DIAMONDS),
        Card::new(CARD_VALUE::ACE as u8, SUIT::HEARTS) 
    ]);

    let hand4 = Hand::new([ 
        Card::new(CARD_VALUE::ACE as u8, SUIT::SPADES), 
        Card::new(CARD_VALUE::ACE as u8, SUIT::DIAMONDS),
        Card::new(CARD_VALUE::FIVE as u8, SUIT::HEARTS) 
    ]);

    let hand5 = Hand::new([ 
        Card::new(CARD_VALUE::JACK as u8, SUIT::SPADES), 
        Card::new(CARD_VALUE::NINE as u8, SUIT::SPADES),
        Card::new(CARD_VALUE::TEN as u8, SUIT::SPADES) 
    ]);

    let hand6 = Hand::new([ 
        Card::new(CARD_VALUE::ACE as u8, SUIT::SPADES), 
        Card::new(CARD_VALUE::KING as u8, SUIT::SPADES),
        Card::new(CARD_VALUE::FIVE as u8, SUIT::SPADES) 
    ]);

    assert_eq!(hand1.get_hand_value(), HAND_VALUE::Straight);
    
    assert_eq!(hand2.get_hand_value(), HAND_VALUE::RoyalFlush);
    
    assert_eq!(hand3.get_hand_value(), HAND_VALUE::ThreeOfAKind);

    assert_eq!(hand4.get_hand_value(), HAND_VALUE::Pair);

    assert_eq!(hand5.get_hand_value(), HAND_VALUE::StraightFlush);

    assert_eq!(hand6.get_hand_value(), HAND_VALUE::Flush);
}