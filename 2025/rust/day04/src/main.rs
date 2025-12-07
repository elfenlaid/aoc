use std::collections::{HashMap, HashSet};
use std::fs;

fn cells_around(cell: &(i32, i32)) -> Vec<(i32, i32)> {
    let (row, column) = &cell;
    let mut cells: Vec<(i32, i32)> = Vec::new();

    for drow in [-1, 0, 1] {
        for dcolumn in [-1, 0, 1] {
            if drow == 0 && dcolumn == 0 {
                continue;
            }

            cells.push((row + drow, column + dcolumn));
        }
    }

    cells
}

fn add_connections(connections: &mut HashMap<(i32, i32), u32>, cell: &(i32, i32)) {
    for neighbour in cells_around(cell) {
        connections
            .entry(neighbour)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
}

fn remove_connections(connections: &mut HashMap<(i32, i32), u32>, cell: &(i32, i32)) {
    for neighbour in cells_around(cell) {
        _ = connections.entry(neighbour).and_modify(|c| *c -= 1);
    }
}

fn part1(input: &str) -> u32 {
    let mut rolls: Vec<(i32, i32)> = Vec::new();

    for (row, line) in (0i32..).zip(input.lines()) {
        for (column, space) in (0i32..).zip(line.chars()) {
            if space == '@' {
                rolls.push((row, column));
            }
        }
    }

    let mut connections: HashMap<(i32, i32), u32> = HashMap::new();

    for roll in &rolls {
        add_connections(&mut connections, roll);
    }

    let mut roll_count: u32 = 0;

    for roll in &rolls {
        let connection_count = connections.get(roll).copied().unwrap_or(0);
        if connection_count < 4 {
            roll_count += 1;
        }
    }

    roll_count
}

fn part2(input: &str) -> u32 {
    let mut rolls: HashSet<(i32, i32)> = HashSet::new();

    for (row, line) in (0i32..).zip(input.lines()) {
        for (column, space) in (0i32..).zip(line.chars()) {
            if space == '@' {
                _ = rolls.insert((row, column));
            }
        }
    }

    let mut connections: HashMap<(i32, i32), u32> = HashMap::new();

    for roll in &rolls {
        add_connections(&mut connections, roll);
    }

    let mut total_roll_count: u32 = 0;

    loop {
        let mut removed_rolls: HashSet<(i32, i32)> = HashSet::new();

        for roll in &rolls {
            let connection_count = connections.get(roll).copied().unwrap_or(0);
            if connection_count < 4 {
                _ = removed_rolls.insert(*roll);
            }
        }

        if removed_rolls.is_empty() {
            break;
        }

        for roll in &removed_rolls {
            rolls.remove(roll);
            remove_connections(&mut connections, roll);
        }

        total_roll_count += removed_rolls.len() as u32;
    }

    total_roll_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn part1_works_with_test() {
        let input = fs::read_to_string("assets/test_input.txt").unwrap();

        assert_eq!(part1(&input), 13)
    }

    #[test]
    fn part2_works_with_test() {
        let input = fs::read_to_string("assets/test_input.txt").unwrap();

        assert_eq!(part2(&input), 43)
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert!(part1(&input) == 1457);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("assets/input.txt").unwrap();
        assert!(part2(&input) == 8310);
    }
}

fn main() {
    let input = fs::read_to_string("assets/test_input.txt").unwrap();

    println!("Part 1 {}", part1(&input));
    println!("Part 2 {}", part2(&input));
}
