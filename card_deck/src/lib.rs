use rand::Rng;
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
#[derive(Debug, PartialEq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]

pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),

}

impl Suit {
    pub fn random() -> Suit {
        let random_index: i32 = rand::thread_rng().gen_range(0..=4);
        match random_index {
        0 => return Suit::Heart,
        1 => return Suit::Diamond,
        2 => return Suit::Spade,
        3 => return Suit::Club,
        _=> panic!()
       }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => return Suit::Heart,
            2 => return Suit::Diamond,
            3 => return Suit::Spade,
            4 => return Suit::Club,
            _=> panic!()
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_index: i32 = rand::thread_rng().gen_range(0..=4);
        match random_index {
        0 => return Rank::Ace,
        1 => return Rank::Jack,
        2 => return Rank::King,
        3 => return Rank::Queen,
      _=> panic!()
       }
    }

    pub fn translate(value: u8) -> Rank {
            match value {
                1 => return Rank::Ace,
                11 => return Rank::Jack,
                12 => return Rank::Queen,
                13 => return Rank::King,
                _=> return Rank::Number(value),
            }
        }
}

pub fn winner_card(card: &Card) -> bool {
    match (&card.rank, &card.suit) {
        (Rank::Ace, Suit::Spade) => return true,
        _=>return false
    }
}