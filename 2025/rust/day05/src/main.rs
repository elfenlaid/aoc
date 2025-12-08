use std::cmp::{Ordering, max};
use std::error::Error;
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "something went wrong: {self:?}")
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, Copy)]
struct CustomRange {
    start: u64,
    end: u64,
}

impl FromStr for CustomRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").ok_or(ParseError::InvalidFormat)?;

        Ok(CustomRange {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
    }
}

impl CustomRange {
    fn compare_value(&self, num: &u64) -> Ordering {
        if *num < self.start {
            return Ordering::Greater;
        }

        if *num > self.end {
            return Ordering::Less;
        }

        Ordering::Equal
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }
}

fn merged_ranges(ranges: &[CustomRange]) -> Vec<CustomRange> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut merged: Vec<CustomRange> = Vec::new();
    let mut current = ranges[0];

    for &range in &ranges[1..] {
        if current.end >= range.start {
            current.end = max(current.end, range.end);
        } else {
            merged.push(current);
            current = range;
        }
    }

    merged.push(current);

    merged
}

fn parse_ranges(input: &str) -> Result<Vec<CustomRange>, ParseError> {
    let mut ranges: Vec<CustomRange> = Vec::new();

    for l in input.lines() {
        let range = l.parse()?;
        ranges.push(range);
    }

    ranges.sort_by_key(|r| r.start);

    Ok(ranges)
}

fn part1(input: &str) -> u64 {
    let (range_input, id_input) = input.split_once("\n\n").unwrap();
    let ids: Vec<u64> = id_input.lines().map(|i| i.parse().unwrap()).collect();
    let ranges = parse_ranges(range_input).unwrap();

    let merged_ranges = merged_ranges(&ranges);

    let fresh_count = ids
        .iter()
        .map(|id| merged_ranges.binary_search_by(|r| r.compare_value(id)))
        .filter(|r| r.is_ok())
        .count();

    fresh_count as u64
}

fn part2(input: &str) -> u64 {
    let (range_input, _) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(range_input).unwrap();

    merged_ranges(&ranges).iter().map(|r| r.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert!(part1(&input) == 739);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert!(part2(&input) == 344486348901788);
    }
}

fn main() {
    let input = fs::read_to_string("assets/test_input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
