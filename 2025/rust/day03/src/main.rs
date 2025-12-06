use std::fs;

fn part1(input: &str) -> u32 {
    let banks: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_joltage: u32 = 0;

    for bank in &banks {
        let mut first_battery: u32 = bank[0];
        let mut second_battery: u32 = bank[1];

        for battery in bank[2..].iter().copied() {
            let current = first_battery * 10 + second_battery;
            let lhs = first_battery * 10 + battery;
            let rhs = second_battery * 10 + battery;

            if lhs > current && lhs >= rhs {
                second_battery = battery;
                continue;
            }

            if rhs > current && rhs > lhs {
                first_battery = second_battery;
                second_battery = battery;
            }
        }

        total_joltage += first_battery * 10 + second_battery;
    }

    total_joltage
}

fn squeeze_position(xs: &[u32]) -> Option<usize> {
    for (i, chunk) in xs.windows(2).enumerate() {
        if chunk[1] > chunk[0] {
            return Some(i);
        }
    }

    None
}

fn part2(input: &str) -> u64 {
    let banks: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_joltage: u64 = 0;

    for bank in &banks {
        let mut batteries: Vec<u32> = bank[0..12].to_vec();

        for &battery in bank[12..].iter() {
            if let Some(position) = squeeze_position(&batteries) {
                _ = batteries.remove(position);
                batteries.push(battery);
            }

            if battery > *batteries.last().unwrap() {
                *batteries.last_mut().unwrap() = battery;
            }
        }

        let formatted_joltage: String = batteries.iter().map(|b| b.to_string()).collect();
        let bank_joltage: u64 = formatted_joltage.parse().unwrap();
        total_joltage += bank_joltage
    }

    total_joltage
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert!(part1(&input) == 17074)
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        let output = part2(&input);
        assert!(output == 169512729575727)
    }
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
