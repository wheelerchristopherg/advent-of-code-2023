use std::cmp::{max, Ordering};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [char; 5],
    bid: i64,
    type_: Type,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    Undefined = 0,
}

impl Hand {
    fn new(input: &str) -> Hand {
        let a = input.split(' ').collect::<Vec<&str>>();
        let cards: [char; 5] = a[0].chars().collect::<Vec<char>>().try_into().unwrap();
        let bid = a[1].parse::<i64>().unwrap();
        let mut hand = Hand {
            cards,
            bid,
            type_: Type::Undefined,
        };
        hand.evaluate_type();
        hand
    }

    fn evaluate_type(&mut self) {
        let mut unique_chars: HashMap<char, usize> = HashMap::new();
        for c in self.cards.iter() {
            if let Some(x) = unique_chars.get_mut(c) {
                *x += 1;
            } else {
                unique_chars.insert(*c, 1);
            }
        }

        let mut pairs = 0;
        let mut triples = 0;
        for (_, count) in unique_chars {
            if count == 5 {
                self.type_ = Type::FiveOfKind;
                return;
            } else if count == 4 {
                self.type_ = Type::FourOfKind;
                return;
            } else if count == 3 {
                triples += 1;
            } else if count == 2 {
                pairs += 1;
            }
        }
        if triples == 1 && pairs == 1 {
            self.type_ = Type::FullHouse;
        } else if triples == 1 {
            self.type_ = Type::ThreeOfKind;
        } else if pairs == 2 {
            self.type_ = Type::TwoPair;
        } else if pairs == 1 {
            self.type_ = Type::OnePair;
        } else {
            self.type_ = Type::HighCard;
        }
    }

    fn compare(&self, other: &Hand) -> Ordering {
        let o = self.type_.cmp(&other.type_);
        // println!("Comparing type: {:?} {:?} {:?}", self.type_, o, other.type_);
        if o != Ordering::Equal {
            return o;
        }
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            let o = Hand::compare_card(c1, c2);
            // println!("Comparing card: {} {:?} {}", c1, o, c2);
            if o != Ordering::Equal {
                return o;
            }
        }
        Ordering::Equal
    }

    fn compare_card(a: &char, b: &char) -> Ordering {
        let strength_order = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        let a_strength = strength_order.iter().position(|x| x == a).unwrap();
        let b_strength = strength_order.iter().position(|x| x == b).unwrap();
        b_strength.cmp(&a_strength)
    }

    fn new_p2(input: &str) -> Hand {
        let a = input.split(' ').collect::<Vec<&str>>();
        let cards: [char; 5] = a[0].chars().collect::<Vec<char>>().try_into().unwrap();
        let bid = a[1].parse::<i64>().unwrap();
        let mut hand = Hand {
            cards,
            bid,
            type_: Type::Undefined,
        };
        hand.evaluate_type_p2();
        hand
    }

    fn evaluate_type_p2(&mut self) {
        let mut unique_chars: HashMap<char, usize> = HashMap::new();
        for c in self.cards.iter() {
            if let Some(x) = unique_chars.get_mut(c) {
                *x += 1;
            } else {
                unique_chars.insert(*c, 1);
            }
        }

        let mut most_matching = 0;
        let mut pairs: usize = 0;
        let mut jokers: usize = 0;
        for (card, count) in unique_chars {
            if card != 'J' {
                if count == 5 {
                    self.type_ = Type::FiveOfKind;
                    return;
                } else {
                    most_matching = max(most_matching, count);
                }
                if count == 2 {
                    pairs += 1;
                }
            } else if card == 'J' {
                jokers = count;
            }
        }

        self.type_ = match (most_matching, jokers, pairs) {
            (5, _, _) => Type::FiveOfKind,
            (4, 1, _) => Type::FiveOfKind,
            (4, 0, _) => Type::FourOfKind,
            (3, 2, _) => Type::FiveOfKind,
            (3, 1, _) => Type::FourOfKind,
            (3, 0, 1) => Type::FullHouse,
            (3, 0, 0) => Type::ThreeOfKind,
            (2, 3, _) => Type::FiveOfKind,
            (2, 2, _) => Type::FourOfKind,
            (2, 1, 2) => Type::FullHouse,
            (2, 1, 1) => Type::ThreeOfKind,
            (2, 0, 2) => Type::TwoPair,
            (2, 0, 1) => Type::OnePair,
            (1, 4, _) => Type::FiveOfKind,
            (1, 3, _) => Type::FourOfKind,
            (1, 2, _) => Type::ThreeOfKind,
            (1, 1, _) => Type::OnePair,
            (1, 0, _) => Type::HighCard,
            (0, 5, 0) => Type::FiveOfKind,
            _ => Type::Undefined,
        };
        assert!(
            self.type_ != Type::Undefined,
            "{:?}, m: {most_matching}, j: {jokers}, p: {pairs}",
            self.cards
        );
    }

    fn compare_p2(&self, other: &Hand) -> Ordering {
        let o = self.type_.cmp(&other.type_);
        // println!("Comparing type: {:?} {:?} {:?}", self.type_, o, other.type_);
        if o != Ordering::Equal {
            return o;
        }
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            let o = Hand::compare_card_p2(c1, c2);
            // println!("Comparing card: {} {:?} {}", c1, o, c2);
            if o != Ordering::Equal {
                return o;
            }
        }
        Ordering::Equal
    }

    fn compare_card_p2(a: &char, b: &char) -> Ordering {
        let strength_order = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];
        let a_strength = strength_order.iter().position(|x| x == a).unwrap();
        let b_strength = strength_order.iter().position(|x| x == b).unwrap();
        b_strength.cmp(&a_strength)
    }
}

pub fn part1(input: &[String]) -> i64 {
    let mut hands = input.iter().map(|x| Hand::new(x)).collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.compare(b));
    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as i64) + 1) * x.bid);
    result
}

pub fn part2(input: &[String]) -> i64 {
    let mut hands = input.iter().map(|x| Hand::new_p2(x)).collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.compare_p2(b));
    hands.iter().for_each(|x| println!("{x:?}"));
    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as i64) + 1) * x.bid);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day7.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day7.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
