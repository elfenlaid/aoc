use std::collections::HashSet;
use std::time::Instant;
use std::{fs, i64};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i64, i64, i64);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Edge<'a> {
    from: &'a Point,
    to: &'a Point,
    distance: i64,
}

impl Point {
    fn distance(&self, to: &Point) -> i64 {
        (self.0 - to.0).abs() + (self.1 - to.1).abs() + (self.2 - to.2).abs()
    }
}

fn main() {
    let input = fs::read_to_string("assets/test_input.txt").unwrap();

    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            let coords: Vec<i64> = l.split(",").map(|c| c.parse().unwrap()).collect();
            Point(coords[0], coords[1], coords[2])
        })
        .collect();

    let now = Instant::now();

    let mut edges: Vec<Edge> = Vec::new();

    for p in &points {
        let mut min_point: Option<&Point> = None;
        let mut min_distance: i64 = i64::MAX;

        for to in &points {
            let distance = p.distance(to);
            if distance == 0 {
                continue;
            }

            if distance < min_distance {
                min_distance = distance;
                min_point = Some(to);
            }
        }

        let edge = Edge {
            from: p,
            to: min_point.unwrap(),
            distance: min_distance,
        };

        edges.push(edge);
    }

    edges.sort_by_key(|e| e.distance);

    let mut circuits: Vec<HashSet<&Point>> = Vec::new();

    for edge in edges {
        let position = circuits
            .iter()
            .position(|c| c.contains(edge.from) || c.contains(edge.to));

        if let Some(circuit) = position.and_then(|p| circuits.get_mut(p)) {
            circuit.insert(edge.from);
            circuit.insert(edge.to);
        } else {
            let circuit = HashSet::from([edge.from, edge.to]);
            circuits.push(circuit);
        }
    }

    circuits.sort_by_key(|c| c.iter().count());
    circuits.reverse();

    for c in &circuits {
        println!("{} {:?}", c.iter().count(), c);
    }

    let total: usize = circuits[..2].iter().map(|c| c.iter().count()).product();
    println!("Total: {}", total);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
