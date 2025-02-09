sleigh::main!();

use std::iter::zip;

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (left, right) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().flat_map(str::parse::<u64>);
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();
    (left, right)
}

pub fn part_one(input: &str) -> Result<u64> {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    let sum = zip(left, right).map(|(a, b)| a.abs_diff(b)).sum();

    Ok(Some(sum))
}

pub fn part_two(input: &str) -> Result<u64> {
    let (left, right) = parse(input);

    let sum = left
        .iter()
        .map(|left| left * right.iter().filter(|right| &left == right).count() as u64)
        .sum();

    Ok(Some(sum))
}
