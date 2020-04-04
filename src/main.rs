use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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
        Deck { cards: v }
    }

    fn total_value(&self) -> u32 {
        self.cards.iter().map(|x| x.0).sum()
    }

    fn average_value(&self) -> f32 {
        let sum = self.total_value() as f32;
        let len = self.cards.len() as f32;
        sum / len
    }

    // A toast pulls 3 cards from the deck.
    fn toast(&mut self) -> Toast {
        assert!(self.cards.len() >= 3);
        Toast(
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
        )
    }
}

#[derive(Debug)]
struct Toast(Card, Card, Card);

impl Toast {
    fn to_values(&self) -> (u32, u32, u32) {
        ((self.0).0, (self.1).0, (self.2).0)
    }
}

#[test]
fn deck_init_state() {
    let d = Deck::new();
    assert_eq!(d.cards.len(), 24);
    assert_eq!(d.average_value(), 3.5);
}

#[test]
fn toast_pulls_three() {
    let mut d = Deck::new();
    let initial_total_value = d.total_value();
    assert_eq!(d.cards.len(), 24);
    let t = d.toast();
    let tvs = t.to_values();
    assert!(tvs.0 > 0 && tvs.1 > 0 && tvs.2 > 0);
    assert!(tvs.0 < 7 && tvs.1 < 7 && tvs.2 < 7);
    assert_eq!(d.cards.len(), 21);
    assert_eq!(d.total_value() + tvs.0 + tvs.1 + tvs.2, initial_total_value);
}

fn main() {
    let mut deck = Deck::new();
    let t = deck.toast();
    println!("Hello, world! {:?}", t);
}
