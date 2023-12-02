use std::char;

pub fn part1(input: &[String]) -> i32 {
    let mut numbers = 0;
    for line in input {
        let mut digits: Vec<char> = vec![];
        for c in line.chars() {
            if c.is_numeric() {
                digits.push(c);
            }
        }
        if !digits.is_empty() {
            let num: String = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            numbers += num.parse::<i32>().unwrap();
        }
    }
    numbers
}

fn find_digits(input: &str) -> i32 {
    let x: &[_] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let mut first_occurance = (0, input.len());
    let mut last_occurance = (0, 0);
    for (i, &s) in x.iter().enumerate() {
        let first = input.find(s);
        let last = input.rfind(s);
        if let (Some(f), Some(l)) = (first, last) {
            if f <= first_occurance.1 {
                first_occurance.0 = (i % 9) + 1;
                first_occurance.1 = f;
            }
            if l >= last_occurance.1 {
                last_occurance.0 = (i % 9) + 1;
                last_occurance.1 = l;
            }
        }
    }
    (first_occurance.0 * 10 + last_occurance.0)
        .try_into()
        .unwrap()
}

pub fn part2(input: &[String]) -> i32 {
    let mut numbers = 0;
    for line in input {
        let num = find_digits(line);
        numbers += num;
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day1.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day1.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_line() {
        let input = "gxjzhvkbcjhscdxhjdqxnhsevenxrdrjbcl5fvlvlxjjvb";
        let result = find_digits(input);
        println!("Result: {result}");
    }

    #[test]
    fn test_line1() {
        let input = "5dxcsvgqkmz";
        let result = find_digits(input);
        println!("Result: {result}");
    }
}
