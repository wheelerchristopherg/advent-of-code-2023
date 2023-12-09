use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(branches: &str) -> Node {
        let a = branches.replace("(", "").replace(")", "");
        let node_ids = a.split(", ").collect::<Vec<_>>();
        Node {
            left: node_ids[0].trim().to_owned(),
            right: node_ids[1].trim().to_owned(),
        }
    }
}

pub fn part1(input: &[String]) -> i64 {
    let directions = input[0].chars().collect::<Vec<_>>();
    let nodes = input[2..]
        .iter()
        .map(|x| x.split('=').collect::<Vec<&str>>())
        .map(|x| (x[0].trim(), Node::new(x[1])))
        .collect::<HashMap<&str, Node>>();

    let mut i: usize = 0;
    let mut current_node = "AAA";
    let mut steps = 0;
    loop {
        if i >= directions.len() {
            i = 0;
        }
        current_node = match directions[i] {
            'L' => &nodes.get(&current_node).unwrap().left,
            'R' => &nodes.get(&current_node).unwrap().right,
            _ => unreachable!(),
        };
        steps += 1;
        i += 1;
        // println!("{steps}: {current_node}");
        if current_node == "ZZZ" {
            break;
        }
    }

    steps
}

#[derive(Debug)]
struct Node2 {
    left: usize,
    right: usize,
}

impl Node2 {
    fn new(input: &str, nodes: &[&str]) -> Node2 {
        let a = input.split(" = ").collect::<Vec<_>>();
        let b = a[1].replace("(", "").replace(")", "");
        let node_ids = b.split(", ").collect::<Vec<_>>();
        let left = nodes.iter().position(|x| *x == node_ids[0]).unwrap();
        let right = nodes.iter().position(|x| *x == node_ids[1]).unwrap();
        Node2 { left, right }
    }
}

fn filter_nodes_on_ending_char(name: &str, index: usize, c: char) -> Option<usize> {
    if name.chars().nth(2).unwrap() == c {
        return Some(index);
    }
    return None;
}

pub fn part2(input: &[String]) -> usize {
    let directions = input[0].chars().collect::<Vec<_>>();
    println!("directions.len() = {}", directions.len());
    let node_names = input[2..].iter().map(|x| &x[0..3]).collect::<Vec<_>>();
    let nodes = input[2..]
        .iter()
        .map(|x| Node2::new(&x, &node_names))
        .collect::<Vec<_>>();

    let all_endings = node_names
        .iter()
        .enumerate()
        .filter_map(|(i, name)| filter_nodes_on_ending_char(name, i, 'Z'))
        .collect::<Vec<_>>();

    let mut i: usize = 0;
    let mut current_nodes = node_names
        .iter()
        .enumerate()
        .filter_map(|(index, name)| filter_nodes_on_ending_char(name, index, 'A'))
        .collect::<Vec<_>>();
    let initial_nodes = current_nodes.clone();
    let initial_node_names = initial_nodes
        .iter()
        .map(|&x| node_names[x])
        .collect::<Vec<_>>();
    println!("{initial_node_names:?}");
    let mut steps = 0;
    let mut stop = initial_node_names.len();
    let mut result: usize = 1;
    loop {
        if i >= directions.len() {
            i = 0;
        }
        for j in 0..current_nodes.len() {
            current_nodes[j] = match directions[i] {
                'L' => nodes[current_nodes[j]].left,
                'R' => nodes[current_nodes[j]].right,
                _ => unreachable!(),
            };
        }
        steps += 1;
        i += 1;
        for (index, &k) in current_nodes.iter().enumerate() {
            if let Some(_) = filter_nodes_on_ending_char(node_names[k], index, 'Z') {
                println!(
                    "{} -> {} in {} steps, i: {i}, steps/i = {}",
                    initial_node_names[index],
                    node_names[k],
                    steps,
                    steps / i
                );
                result *= steps / directions.len();
                stop -= 1;
                if stop == 0 {
                    return result * directions.len();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/day8.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/day8.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
