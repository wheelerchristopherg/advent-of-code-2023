pub fn part1(input: &[String]) -> i32 {
    let mut possible_id_summation: i32 = 0;
    for line in input {
        let a: Vec<_> = line.split(": ").collect();
        let b: Vec<_> = a[0].split(' ').collect();
        let game_id = b[1];
        let draws: Vec<_> = a[1].split("; ").collect();
        println!("{}", line);
        let mut possible = true;
        'outer: for draw in draws {
            let cubes_drawn: Vec<_> = draw.split(", ").collect();
            for cubes in cubes_drawn {
                println!("{}", cubes);
                let parts: Vec<_> = cubes.split(' ').collect();
                let count: i32 = parts[0].parse().unwrap();
                let color = parts[1];
                if (color == "red" && count > 12)
                    || (color == "green" && count > 13)
                    || (color == "blue" && count > 14)
                {
                    possible = false;
                    println!("this one was impossible");
                    break 'outer;
                }
            }
        }
        if possible {
            possible_id_summation += game_id.parse::<i32>().unwrap();
        }
    }
    possible_id_summation
}

pub fn part2(input: &[String]) -> i32 {
    let mut power_summation: i32 = 0;
    for line in input {
        let a: Vec<_> = line.split(": ").collect();
        let draws: Vec<_> = a[1].split("; ").collect();
        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;
        println!("{}", line);
        for draw in draws {
            let cubes_drawn: Vec<_> = draw.split(", ").collect();
            for cubes in cubes_drawn {
                println!("{}", cubes);
                let parts: Vec<_> = cubes.split(' ').collect();
                let count: i32 = parts[0].parse().unwrap();
                let color = parts[1];
                match color {
                    "red" if count > red_min => red_min = count,
                    "green" if count > green_min => green_min = count,
                    "blue" if count > blue_min => blue_min = count,
                    _ => (),
                };
            }
        }
        power_summation += red_min * green_min * blue_min;
    }
    power_summation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day2.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day2.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
