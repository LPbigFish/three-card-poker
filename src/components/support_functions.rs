use crate::components::hand::{Hand};

pub fn is_flush(hand: &Hand) -> bool {
    match &hand.holder {
        [a, b, c] => a.suit == b.suit && a.suit == c.suit
    }
}

pub fn is_straight(hand: &Hand) -> bool {
    if hand.has_ace {
        match &hand.holder {
            [a, b, ..] => return (a.value == 2 && b.value == 3) || (a.value == 12 && b.value == 13)
        }
    }
    hand.holder[0].value + 1 == hand.holder[1].value && hand.holder[1].value + 1 == hand.holder[2].value
}

pub fn is_royal_flush(hand: &Hand) -> bool {
    if !hand.has_ace || hand.holder[1].value != 13 || hand.holder[0].value != 12 {
        return false;
    }

    if !is_flush(hand) {
        return false;
    }

    true
}

pub fn is_straight_flush(hand: &Hand) -> bool {
    return is_flush(hand) && is_straight(hand)
}

pub fn is_three_of_a_kind(hand: &Hand) -> Option<u8> {
    match &hand.holder {
        [a, b, c] => if a.value == b.value && a.value == c.value { return Some(a.value); }
    }
    None
}

pub fn get_a_pair(hand: &Hand) -> Option<u8> {
    let res;
    match &hand.holder {
        [a,b , c] => if a.value == b.value { res = [a, b] } else if b.value == c.value { res = [b, c] } else { return None; }
    }

    Some(res[0].value)
}

pub fn get_highest_card_in_pair(hand: &Hand) -> u8 {
    match &hand.holder {
        [a,b , c] => if a.value == b.value { c.value } else if b.value == c.value { a.value } else { 0 }
    }
}