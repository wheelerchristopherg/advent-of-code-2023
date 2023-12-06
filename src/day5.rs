use std::collections::HashMap;

#[derive(Debug)]
struct Mapping {
    source: i64,
    destination: i64,
    length: i64,
}
impl Mapping {
    fn new(input: &str) -> Mapping {
        let values = input
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Mapping {
            source: values[1],
            destination: values[0],
            length: values[2],
        }
    }

    fn map(&self, value: i64) -> Option<i64> {
        if value >= self.source && value < self.source + self.length {
            let diff = value - self.source;
            return Some(self.destination + diff);
        }
        return None;
    }

    fn map2(&self, start: i64, length: i64) -> Vec<(i64, i64, bool)> {
        let mut output = vec![];
        if length == 1 {
            let new_start = self.map(start);
            if let Some(x) = new_start {
                output.push((x, 1, true));
            } else {
                output.push((start, 1, false));
            }
        } else if start == self.source && length == self.length {
            output.push((start, length, false));
        } else if start >= self.source && start + length - 1 < self.source + self.length {
            output.push((self.destination + (start - self.source), length, true));
        // entire range is within the mapped range
        } else if start >= self.source
            && start < self.source + self.length
            && start + length > self.source + self.length
        {
            let new_length = (self.source + self.length) - start;
            output.push((self.destination + (start - self.source), new_length, true)); // section of range mapped to new range
            output.push((self.source + self.length, length - new_length, false));
        // section of range that did not map
        } else if start < self.source
            && start + length > self.source
            && start + length <= self.source + self.length
        {
            let new_length = self.source - start;
            output.push((start, new_length, false)); // section before mapped range
            output.push((self.destination, length - new_length, true)); // section of range that mapped
        } else if start < self.source && start + length > self.source + self.length {
            let length1 = self.source - start;
            let length2 = self.length;
            let length3 = (start + length) - (self.source + self.length);
            output.push((start, length1, false)); // section before mapped range
            output.push((self.destination, length2, true)); // mapped range
            output.push((start + length1 + length2, length3, false)); // section after mapped range
        } else {
            output.push((start, length, false));
        }
        output
    }
}

pub fn part1(input: &[String]) -> i64 {
    let mut seeds = vec![];
    let mut state = "seeds";
    let states = [
        "to-soil",
        "to-fertilizer",
        "to-water",
        "to-light",
        "to-temperature",
        "to-humidity",
        "to-location",
    ];
    let mut mappings = HashMap::new();
    'outer: for line in input.iter() {
        if line.is_empty() {
            state = "";
            continue 'outer;
        }
        if state.is_empty() {
            for &mapping in states.iter() {
                if line.contains(mapping) {
                    state = mapping;
                    mappings.insert(state, vec![]);
                    continue 'outer;
                }
            }
        }

        if state == "seeds" {
            seeds = line.split(':').collect::<Vec<_>>()[1]
                .trim()
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        } else {
            let map = Mapping::new(line);
            mappings.get_mut(&state).unwrap().push(map);
        }
    }

    let mut min_location = i64::MAX;
    for &seed in seeds.iter() {
        let mut value = seed;
        println!("seed: {value}");
        'conversion: for conversion in states.iter() {
            for range in mappings.get(conversion).unwrap().iter() {
                println!("\t{range:?}");
                if let Some(x) = range.map(value) {
                    value = x;
                    continue 'conversion;
                }
            }
            println!("\t{conversion}: {value}");
        }
        if value < min_location {
            min_location = value;
        }
    }
    min_location
}

pub fn part2(input: &[String]) -> i64 {
    let mut seeds = vec![];
    let mut state = 20usize;
    let states = [
        "to-soil",
        "to-fertilizer",
        "to-water",
        "to-light",
        "to-temperature",
        "to-humidity",
        "to-location",
    ];

    let mut mappings = vec![];
    'outer: for line in input.iter() {
        if line.is_empty() {
            state = 30;
            continue 'outer;
        }
        if state == 30 {
            for (i, &mapping) in states.iter().enumerate() {
                if line.contains(mapping) {
                    state = i;
                    mappings.push(vec![]);
                    continue 'outer;
                }
            }
        }

        if state == 20 {
            seeds = line.split(':').collect::<Vec<_>>()[1]
                .trim()
                .split(' ')
                .collect::<Vec<_>>()
                .windows(2)
                .step_by(2)
                .map(|x| (x[0].parse::<i64>().unwrap(), x[1].parse::<i64>().unwrap()))
                .collect::<Vec<_>>();
        } else {
            mappings[state].push(Mapping::new(line));
        }
    }
    let mut results: [Vec<(i64, i64)>; 8] = [
        seeds,
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let mut min_location = i64::MAX;

    let mut extras = vec![];
    let mut index = 0usize;
    while index < 7usize {
        let source = if extras.is_empty() {
            results[index].clone()
        } else {
            let extras_clone = extras.clone();
            extras.clear();
            extras_clone
        };
        'outer: for &(start, length) in source.iter() {
            let mut mapped = false;
            'range: for range in mappings[index].iter() {
                if mapped {
                    continue 'outer;
                }
                let mapped_range = range.map2(start, length);
                if mapped_range.len() == 1 && !mapped_range[0].2 {
                    continue 'range;
                } else {
                    mapped = true;
                }
                for (x, y, b) in mapped_range {
                    if b {
                        results[index + 1].push((x, y));
                    } else if !extras.contains(&(x, y)) {
                        extras.push((x, y));
                    }
                }
            }
            if !mapped {
                results[index + 1].push((start, length));
            }
        }
        if extras.is_empty() {
            index += 1;
        }
    }

    for &(start, _) in results[6].iter() {
        if start < min_location {
            min_location = start;
        }
    }

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day5.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day5.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_mapping_map2_1() {
        let mapper = Mapping::new("0 68 1");
        let result = mapper.map2(68, 1);
        let expected: Vec<(i64, i64, bool)> = vec![(0, 1, true)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_2() {
        let mapper = Mapping::new("50 5 8");
        let result = mapper.map2(3, 15);
        let expected: Vec<(i64, i64, bool)> = vec![(3, 2, false), (50, 8, true), (13, 5, false)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_3() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(5, 10);
        let expected: Vec<(i64, i64, bool)> = vec![(5, 5, false), (100, 5, true)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_4() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(10, 10);
        let expected: Vec<(i64, i64, bool)> = vec![(100, 5, true), (15, 5, false)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_5() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(11, 2);
        let expected: Vec<(i64, i64, bool)> = vec![(101, 2, true)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_6() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(11, 4);
        let expected: Vec<(i64, i64, bool)> = vec![(101, 4, true)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_7() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(10, 4);
        let expected: Vec<(i64, i64, bool)> = vec![(100, 4, true)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_8() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(15, 4);
        let expected: Vec<(i64, i64, bool)> = vec![(15, 4, false)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapping_map2_9() {
        let mapper = Mapping::new("100 10 5");
        let result = mapper.map2(5, 5);
        let expected: Vec<(i64, i64, bool)> = vec![(5, 5, false)];
        println!("{:?}", result);
        assert_eq!(result, expected);
    }
}
