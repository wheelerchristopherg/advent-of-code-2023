use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn read_input(filename: &str) -> Vec<String> {
    let input = File::open(filename).expect("failed to read file");
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect()
}

pub fn split_and_parse<T>(input: &str, separator: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}
