use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

fn d6() -> u32 {
    let mut rng = thread_rng();
    let d6: u32 = rng.gen_range(1, 7);
    return d6;
}

#[derive(Debug)]
struct Card(u32);

impl Card {
    fn new(value: u32) -> Card {
        assert!(value > 0 && value < 7);
        Card(value)
    }

    fn value(&self) -> u32 {
        self.0
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
        Toast {
            cards: vec![
                self.cards.pop().unwrap(),
                self.cards.pop().unwrap(),
                self.cards.pop().unwrap(),
            ],
        }
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[derive(Debug)]
struct Toast {
    cards: Vec<Card>,
}

impl Toast {
    fn to_values(&self) -> Vec<u32> {
        self.cards.iter().map(|x| x.0).collect()
    }

    fn sum(&self) -> u32 {
        self.to_values().iter().sum()
    }
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn try_clear_first(&mut self, r0: u32, r1: u32) {
        // Given the two dice, if either matches the front then remove it.
        let front = self.cards.first().unwrap().value();
        if r0 == front || r1 == front {
            self.cards.remove(0);
        }
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
    assert_eq!(t.cards.len(), 3);
    assert_eq!(d.cards.len(), 21);
    assert_eq!(d.total_value() + t.sum(), initial_total_value);
}

/// An attempt either wins and gives the score back,
/// or it loses and gives a None back.
fn attempt(deck: &mut Deck) -> Option<u32> {
    // First you toast (Deal 3 cards)
    let mut t = deck.toast();

    // Number of rolls you get is the total.
    let mut rolls = t.sum();

    // It's also the score we get if we beat the toast
    let score_if_win = t.sum();
    while rolls > 0 {
        rolls -= 1;
        // You get two dice on a roll, use them to try to clear a card
        t.try_clear_first(d6(), d6());
        // See if we've beaten all 3 cards
        if t.is_empty() {
            return Some(score_if_win);
        }
    }
    return None;
}

fn max_score_attempt() -> bool {
    let mut deck = Deck::new();
    loop {
        if let None = attempt(&mut deck) {
            return false;
        }
        if deck.is_empty() {
            return true;
        }
    };
}

fn main() {
    let attempts = 10000;
    let wins = (0..attempts)
        .map(|_| max_score_attempt())
        .filter(|b| *b)
        .count();
    println!("{} of {} attempts won", wins, attempts);
}
