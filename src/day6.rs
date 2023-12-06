pub fn part1(input: &[String]) -> i32 {
    let times = input[0].split(':').collect::<Vec<_>>()[1]
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let distances = input[1].split(':').collect::<Vec<_>>()[1]
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("times: {times:?}");
    println!("distances: {distances:?}");

    let mut result = 1;
    for index in 0..times.len() {
        let mut count = 0;
        println!("race: {index}");
        for press_time in 0..times[index] {
            let travel_time = times[index] - press_time;
            let distance = press_time * travel_time;
            if distance > distances[index] {
                count += 1
            }
            println!("press_time: {press_time}, travel_time: {travel_time}, distance: {distance}, won: {}", distance > distances[index]);
        }
        result *= count;
    }

    result
}

pub fn part2(input: &[String]) -> i64 {
    let time = input[0].split(':').collect::<Vec<_>>()[1]
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let distance = input[1].split(':').collect::<Vec<_>>()[1]
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    println!("times: {time:?}");
    println!("distances: {distance:?}");

    let mut count = 0;
    for press_time in 0..time {
        let travel_time = time - press_time;
        let new_distance = press_time * travel_time;
        if new_distance > distance {
            count += 1
        }
        
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day6.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day6.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
