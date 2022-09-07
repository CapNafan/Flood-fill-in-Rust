extern crate queues;

use queues::*;
use std::collections::HashSet;

fn main() {
    flood_fill(1000, 1000);
}

fn flood_fill(x: i32, y: i32) {
    let mut points: Queue<(i32, i32)> = Queue::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let point = (x, y);
    points.add(point);

    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while points.size() != 0 {
        let removed_point = points.remove();
        let (x1, y1) = removed_point.unwrap();

        for offset in &offsets {
            let (x_coord, y_coord) = (x1 + offset.0, y1 + offset.1);

            if valid_coords(x_coord, y_coord) && !visited.contains(&(x_coord, y_coord)) {
                points.add((x_coord, y_coord));
                visited.insert((x_coord, y_coord));
            }
        }
    }
    println!("number of cells ant could visit is {}", visited.len());
}

fn valid_coords(x: i32, y: i32) -> bool {
    let x_sum: i32 = parse_integer(x).iter().sum();
    let y_sum: i32 = parse_integer(y).iter().sum();
    x_sum + y_sum <= 25
}

fn parse_integer(mut num: i32) -> Vec<i32> {
    let mut ints: Vec<i32> = Vec::new();
    while num.abs() > 0 {
        ints.push(num % 10);
        num /= 10;
    }
    ints
}
