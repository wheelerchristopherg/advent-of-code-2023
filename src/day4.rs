pub fn part1(input: &[String]) -> i32 {
    let mut total_score = 0;
    for line in input {
        let a = line.split(':').collect::<Vec<_>>()[1]
            .trim()
            .split('|')
            .collect::<Vec<_>>();
        let winning_numbers = a[0]
            .split(' ')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect::<Vec<_>>();
        let numbers = a[1]
            .split(' ')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect::<Vec<_>>();
        let mut score = 0;
        for number in numbers {
            if winning_numbers.contains(&number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        total_score += score;
    }
    total_score
}

struct ScratchCard {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    cards_won: Vec<i32>,
    count: i32,
}

impl ScratchCard {
    fn new(input: &str) -> ScratchCard {
        let id_and_numbers = input.split(':').collect::<Vec<_>>();
        let id = id_and_numbers[0]
            .trim()
            .split(' ')
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let numbers = id_and_numbers[1].trim().split('|').collect::<Vec<_>>();
        let winning_numbers = numbers[0]
            .split(' ')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect::<Vec<_>>();
        let numbers = numbers[1]
            .split(' ')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect::<Vec<_>>();
        ScratchCard {
            id,
            winning_numbers,
            numbers,
            cards_won: vec![],
            count: 1,
        }
    }

    fn evaluate(&mut self) {
        let mut score = 0;
        for number in self.numbers.iter() {
            if self.winning_numbers.contains(number) {
                score += 1;
                for _ in 0..self.count {
                    self.cards_won.push(self.id + score);
                }
            }
        }
    }
}

pub fn part2(input: &[String]) -> i32 {
    let mut scratch_cards: Vec<ScratchCard> = input
        .iter()
        .map(|x| ScratchCard::new(x))
        .collect::<Vec<_>>();
    for i in 0..scratch_cards.len() {
        scratch_cards[i].evaluate();
        for &id_copy in scratch_cards[i].cards_won.clone().iter() {
            scratch_cards[(id_copy - 1) as usize].count += 1;
        }
    }

    let total_cards = scratch_cards.iter().fold(0, |acc, x| acc + x.count);
    total_cards
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day4.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day4.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
