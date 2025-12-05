use std::{collections::HashSet, fs};

fn digit_count(number: u64) -> u32 {
    (number as f64).log10() as u32 + 1
}

fn part1(input: &str) -> u64 {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    let mut sum: u64 = 0;

    for (range_start, range_end) in ranges {
        let even_start = {
            let count = digit_count(range_start);

            if count.is_multiple_of(2) {
                range_start
            } else {
                10_u64.pow(count)
            }
        };

        let even_end = {
            let count = digit_count(range_end);

            if count.is_multiple_of(2) {
                range_end
            } else {
                10_u64.pow(count - 1) - 1
            }
        };

        let half_digit_count = digit_count(even_start) / 2;
        let shifter = 10_u64.pow(half_digit_count);
        let mut prefix = even_start / shifter;

        loop {
            let number = prefix * shifter + prefix;

            if number > even_end {
                break;
            }

            if number >= range_start {
                sum += number;
            }

            prefix += 1;
        }
    }

    println!("Part 1: {sum}");

    sum
}

fn replicate_in(number: u64, host_digit_count: u32) -> Option<u64> {
    let number_digit_count = digit_count(number);

    let host_fits_number_evenly = host_digit_count.is_multiple_of(number_digit_count);
    if !host_fits_number_evenly {
        return None;
    }

    let fit_count = host_digit_count / number_digit_count;
    if fit_count <= 1 {
        return None;
    }

    Some(replicate(number, fit_count))
}

fn replicate(number: u64, times: u32) -> u64 {
    let count = digit_count(number);
    let mut replication: u64 = 0;

    for i in 0..times {
        replication += number * 10_u64.pow(count * i);
    }

    replication
}

fn part2(input: &str) -> u64 {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    let mut ids: HashSet<u64> = HashSet::new();

    for (start, end) in ranges {
        let start_digit_count = digit_count(start);
        let end_digit_count = digit_count(end);

        for digit_count in start_digit_count..=end_digit_count {
            let to_half_end = 10_u64.pow(digit_count / 2) - 1;

            for filler in 1..=to_half_end {
                let Some(invalid_id) = replicate_in(filler, digit_count) else {
                    continue;
                };

                if (start..=end).contains(&invalid_id) {
                    _ = ids.insert(invalid_id);
                }
            }
        }
    }

    let sum = ids.iter().sum();
    println!("Part 2: {sum}");
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert_eq!(part2(&input), 46769308485);
    }

    #[test]
    fn part2_works_with_test_input() {
        let input = fs::read_to_string("assets/test_input.txt").unwrap();
        assert_eq!(part2(&input), 4174379265);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert_eq!(part1(&input), 31000881061);
    }

    #[test]
    fn part1_works_with_test_input() {
        let input = fs::read_to_string("assets/test_input.txt").unwrap();

        assert_eq!(part1(&input), 1227775554);
    }
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    part1(&input);
    part2(&input);
}
