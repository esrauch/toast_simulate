use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

fn d6() -> u32 {
    let mut rng = thread_rng();
    let d6: u32 = rng.gen_range(1, 7);
    return d6;
}

#[derive(Debug)]
struct TwoDice(u32, u32);

impl TwoDice {
    fn roll() -> TwoDice {
        TwoDice(d6(), d6())
    }
}

#[derive(Debug)]
struct Card(u32);

impl Card {
    fn new(value: u32) -> Card {
        assert!(value > 0 && value < 7);
        Card(value)
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        // Initial deck has the cards 1-6 each 4 times (one for each suit).
        let mut v = Vec::new();
        for value in 1..7 {
            for _suit in 1..5 {
                v.push(Card::new(value));
            }
        }
        let mut rng = thread_rng();
        v.shuffle(&mut rng);
        Deck {
            cards: v,
        }
    }

    fn average_value(&self) -> f32 {
        let sum = self.cards.iter().map(|x| x.0).sum::<u32>() as f32;
        let len = self.cards.len() as f32;
        sum / len
    }

    // A toast pulls 3 cards from the deck.
    fn toast(&mut self) -> (Card, Card, Card) {
        assert!(self.cards.len() >= 3);
        (
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
        )
    }
}

#[test]
fn deck_init_state() {
    let d = Deck::new();
    assert_eq!(d.cards.len(), 24);
    assert_eq!(d.average_value(), 3.5);
}


fn main() {
    let mut deck = Deck::new();

    let t = deck.toast();
    println!("Hello, world! {:?}", t);
}
