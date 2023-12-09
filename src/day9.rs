pub fn part1(input: &[String]) -> i64 {
    let mut total = 0;
    for line in input.iter() {
        let parsed = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut working = parsed.clone();
        let mut sum = *working.last().unwrap();
        while !working.iter().all(|x| *x == 0) {
            println!("{working:?}");
            working = working.windows(2).map(|x| x[1] - x[0]).collect();
            sum += *working.last().unwrap();
        }
        println!("{working:?}");
        println!("{sum}");
        total += sum;
    }
    total
}

pub fn part2(input: &[String]) -> i64 {
    let mut total = 0;
    for line in input.iter() {
        let parsed = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut working = parsed.clone();
        let mut firsts = vec![working[0]];
        while !working.iter().all(|x| *x == 0) {
            println!("{working:?}");
            working = working.windows(2).map(|x| x[1] - x[0]).collect();
            firsts.push(working[0]);
        }
        println!("{working:?}");
        while firsts.len() > 1 {
            let a = firsts.pop().unwrap();
            let b = firsts.pop().unwrap();
            firsts.push(b - a);
            println!("firsts: {firsts:?}");
        }
        total += firsts[0];
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day9.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day9.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
