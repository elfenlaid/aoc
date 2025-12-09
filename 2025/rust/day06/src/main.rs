use std::{collections::HashMap, fs};

fn parse_line_numbers(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn apply(operator: &str, lhs: &u64, rhs: &u64) -> u64 {
    match operator {
        "*" => lhs * rhs,
        "+" => lhs + rhs,
        _ => panic!(),
    }
}

fn part1(input: &str) -> u64 {
    let mut i = input.lines().rev();

    let operators: Vec<&str> = i.next().unwrap().split_whitespace().collect();
    let mut values = parse_line_numbers(i.next().unwrap());

    for line in i {
        for (i, number) in parse_line_numbers(line).iter().enumerate() {
            let operator = operators[i];
            values[i] = apply(operator, &values[i], number);
        }
    }

    values.iter().sum()
}

fn part2(input: &str) -> u64 {
    let mut input_iter = input.lines().rev();
    let operator_line = input_iter.next().unwrap();

    let mut numbers: HashMap<usize, String> = HashMap::new();

    for line in input_iter.rev() {
        for (number_index, char) in line.chars().enumerate() {
            if !char.is_whitespace() {
                numbers
                    .entry(number_index)
                    .and_modify(|n| n.push(char))
                    .or_insert(char.to_string());
            }
        }
    }

    let mut current_operator: Option<String> = None;
    let mut column_sum: u64 = 0;
    let mut sum: u64 = 0;

    for (number_index, c) in operator_line.chars().enumerate() {
        if c == '*' || c == '+' {
            sum += column_sum;
            column_sum = if c == '*' { 1 } else { 0 };
            current_operator = Some(c.to_string());
        }

        if let Some(number) = numbers.get(&number_index).map(|n| n.parse().unwrap()) {
            column_sum = apply(current_operator.as_ref().unwrap(), &column_sum, &number);
        }
    }

    sum += column_sum;

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert_eq!(part1(&input), 6725216329103)
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert_eq!(part2(&input), 10600728112865)
    }
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
