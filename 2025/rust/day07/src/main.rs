use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(input: &str) -> u32 {
    let mut input_iter = input.lines();

    let start_position = input_iter
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    let mut beams = HashSet::from([start_position]);
    let mut split_count: u32 = 0;

    for line in input_iter {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beams.contains(&i) {
                split_count += 1;
                beams.remove(&i);
                beams.insert(i + 1);
                beams.insert(i - 1);
            }
        }
    }

    split_count
}

fn part2(input: &str) -> u64 {
    let mut input_iter = input.lines();

    let start_position = input_iter
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    let mut beams: HashMap<usize, u64> = HashMap::new();
    beams.insert(start_position, 1);

    for line in input_iter {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beams.contains_key(&i) {
                let value = beams.remove(&i).unwrap();

                beams
                    .entry(i - 1)
                    .and_modify(|v| *v += value)
                    .or_insert(value);

                beams
                    .entry(i + 1)
                    .and_modify(|v| *v += value)
                    .or_insert(value);
            }
        }
    }

    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert_eq!(part1(&input), 1642)
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert_eq!(part2(&input), 47274292756692)
    }
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
