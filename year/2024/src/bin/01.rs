sleigh::main!();

use std::iter::zip;

fn parse(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    let pairs: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid input: missing left number"))?
                .parse()?;
            let right = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid input: missing right number"))?
                .parse()?;
            Ok((left, right))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;

    let (left, right): (Vec<u64>, Vec<u64>) = pairs.into_iter().unzip();

    Ok((left, right))
}

pub fn part_one(input: &str) -> Result<u64> {
    let (mut left, mut right) = parse(input)?;

    left.sort_unstable();
    right.sort_unstable();

    let sum = zip(left, right).map(|(a, b)| a.abs_diff(b)).sum();

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u64> {
    let (left, right) = parse(input)?;

    let sum = left
        .iter()
        .map(|left| left * right.iter().filter(|right| left == *right).count() as u64)
        .sum();

    Ok(sum)
}
