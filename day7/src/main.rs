use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use itertools::Itertools;

#[derive(Eq)]
struct Hand {
    cards: [usize; 5],
    hand_type: usize,
    bid: usize
}

impl Hand {

    fn _hand_type(cards: &[usize; 5], jokers: bool) -> usize{
        let mut map = cards
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                map
            });

        if jokers { // if jokers reassign in hashmap then match
            if let Some(num_jokers) = map.remove(&9) {
                match map.iter().max_by_key(|(_, &v) | v) {
                    Some(k) => *map.get_mut(&k.0.to_owned()).unwrap() += num_jokers,
                    None => {map.entry(12).or_insert(5);} // all jokers, pick aces
                };
            }
        }

        let frequencies: Vec<usize> = map.iter().map(|(_, &freq)| freq).sorted().collect();
        match frequencies[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1
        }
    }

    fn _get_card_label(x: char) -> usize {
        let valid_chars = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
        valid_chars.iter().position(|&c| c == x).unwrap()
    }
    pub fn new(line: &str) -> Self {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let cards = card_str.chars().map(Hand::_get_card_label).collect::<Vec<usize>>().try_into().unwrap();
        Hand {
            cards,
            hand_type: Hand::_hand_type(&cards, false),
            bid: bid_str.parse::<usize>().unwrap()
        }
    }

    pub fn joker_rescore(&mut self) {
        self.hand_type = Hand::_hand_type(&self.cards, true);
        for c in &mut self.cards {
            *c = if *c == 9 { 0 } else { *c + 1 };
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type { // we could eliminate this check
            for (a, b) in zip(self.cards, other.cards) {
                if a != b {return false}
            }
            return true
        }
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in zip(self.cards, other.cards) {
                    if a != b {return a.cmp(&b)}
                }
                Ordering::Equal
            }
        }
    }
}

fn camel_cards(data: &str) -> (usize, usize) {
    let mut hands: Vec<Hand> = data.lines().map(Hand::new).sorted().collect();
    let p1 = hands.iter().enumerate().map(|(idx, h)| h.bid * (idx + 1)).sum();
    for h in &mut hands {
        h.joker_rescore();
    }
    let p2 = hands.iter().sorted().enumerate().map(|(idx, h)| h.bid * (idx + 1)).sum();
    (p1, p2)
}

fn main() {
    let data = read_to_string("C:/Users/Fabian/Code/AdventOfCode/2023/day7/input").unwrap();
    let result = camel_cards(data.as_str());

    println!("{result:?} lol");

}