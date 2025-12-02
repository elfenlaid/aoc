use std::fs;

fn part1(input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).unwrap();

    let rotations = input.lines().map(|line| {
        let direction = if line.starts_with("L") { -1 } else { 1 };
        let distance: i32 = line[1..].parse().unwrap();

        direction * distance
    });

    let mut dial = 50;
    let mut zeros = 0;

    for rotation in rotations {
        dial = (dial + rotation) % 100;

        if dial == 0 {
            zeros += 1;
        }
    }

    println!("Part 1: {zeros}");

    zeros
}

fn part2(input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).unwrap();

    let rotations = input.lines().map(|line| {
        let direction = if line.starts_with("L") { -1 } else { 1 };
        let distance: i32 = line[1..].parse().unwrap();

        direction * distance
    });

    let mut dial = 50;
    let mut zeros = 0;

    for rotation in rotations {
        let previous_dial = dial;
        let overshoot_dial = dial + rotation % 100;
        dial = overshoot_dial % 100;

        // Account for full rotations
        zeros += rotation.abs() / 100;

        let clicked_on_dead_zero = dial == 0;
        let clicked_on_overshoot = overshoot_dial > 100 || overshoot_dial < -100;
        let clicked_clockwise = previous_dial < 0 && overshoot_dial > 0;
        let clicked_counterclockwise = previous_dial > 0 && overshoot_dial < 0;

        let clicked_with_part_rotation = clicked_clockwise
            || clicked_counterclockwise
            || clicked_on_overshoot
            || clicked_on_dead_zero;

        if clicked_with_part_rotation {
            zeros += 1;
        }
    }

    println!("Part 2: {zeros}");

    zeros
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn part1_works() {
        let zero = part1("assets/input.txt");
        check!(zero == 1034);
    }

    #[test]
    fn part2_works() {
        let zeros = part2("assets/input.txt");
        check!(zeros == 6166);
    }
}

fn main() {
    let input = "assets/input.txt";

    part1(input);
    part2(input);
}
