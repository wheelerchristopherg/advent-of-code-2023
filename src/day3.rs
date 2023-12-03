use std::char;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

pub fn part1(input: &[String]) -> i32 {
    let length = input[0].len();
    let mut visited_coords: HashSet<Pos> = HashSet::new();
    let adjacent_positions = [
        Pos { x: -1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: 1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: 1, y: 0 },
        Pos { x: -1, y: 1 },
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
    ];
    let mut part_number_sum = 0;
    for (y, s) in input.iter().enumerate() {
        for (x, c) in s.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                for neighbor in adjacent_positions.iter() {
                    let x: i32 = (x as i32) + neighbor.x;
                    let y: i32 = (y as i32) + neighbor.y;
                    if x < 0 || y < 0 || visited_coords.contains(&Pos { x, y }) {
                        continue;
                    }
                    let c: char = input[y as usize].chars().nth(x as usize).unwrap();
                    if c.is_numeric() {
                        visited_coords.insert(Pos { x, y });
                        let mut i = x - 1;
                        let mut left_bound: usize = 0;
                        let mut right_bound: usize = length;
                        while i >= 0 {
                            if input[y as usize]
                                .chars()
                                .nth(i as usize)
                                .unwrap()
                                .is_numeric()
                            {
                                visited_coords.insert(Pos { x: i, y });
                                i -= 1;
                            } else {
                                left_bound = (i + 1) as usize;
                                break;
                            }
                        }
                        i = x + 1;
                        while i < length as i32 {
                            if input[y as usize]
                                .chars()
                                .nth(i as usize)
                                .unwrap()
                                .is_numeric()
                            {
                                visited_coords.insert(Pos { x: i, y });
                                i += 1;
                            } else {
                                right_bound = i as usize;
                                break;
                            }
                        }
                        let part_number = &input[y as usize][left_bound..right_bound];
                        part_number_sum += part_number.parse::<i32>().unwrap();
                    }
                }
            }
        }
    }
    part_number_sum
}

pub fn part2(input: &[String]) -> i32 {
    let length = input[0].len();
    let mut visited_coords: HashSet<Pos> = HashSet::new();
    let adjacent_positions = [
        Pos { x: -1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: 1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: 1, y: 0 },
        Pos { x: -1, y: 1 },
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
    ];
    let mut gear_ratio_sum = 0;
    for (y, s) in input.iter().enumerate() {
        for (x, c) in s.chars().enumerate() {
            if c == '*' {
                let mut gear_ratio = 1;
                let mut gear_count = 0;
                for neighbor in adjacent_positions.iter() {
                    let x: i32 = (x as i32) + neighbor.x;
                    let y: i32 = (y as i32) + neighbor.y;
                    if x < 0 || y < 0 || visited_coords.contains(&Pos { x, y }) {
                        continue;
                    }
                    let c: char = input[y as usize].chars().nth(x as usize).unwrap();
                    if c.is_numeric() {
                        visited_coords.insert(Pos { x, y });
                        let mut i = x - 1;
                        let mut left_bound: usize = 0;
                        let mut right_bound: usize = length;
                        while i >= 0 {
                            if input[y as usize]
                                .chars()
                                .nth(i as usize)
                                .unwrap()
                                .is_numeric()
                            {
                                visited_coords.insert(Pos { x: i, y });
                                i -= 1;
                            } else {
                                left_bound = (i + 1) as usize;
                                break;
                            }
                        }
                        i = x + 1;
                        while i < length as i32 {
                            if input[y as usize]
                                .chars()
                                .nth(i as usize)
                                .unwrap()
                                .is_numeric()
                            {
                                visited_coords.insert(Pos { x: i, y });
                                i += 1;
                            } else {
                                right_bound = i as usize;
                                break;
                            }
                        }
                        let part_number = input[y as usize][left_bound..right_bound]
                            .parse::<i32>()
                            .unwrap();
                        gear_count += 1;
                        println!("{} {}", gear_count, part_number);
                        gear_ratio *= part_number;
                    }
                }
                if gear_count == 2 {
                    gear_ratio_sum += gear_ratio;
                }
            }
        }
    }
    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day3.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day3.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
