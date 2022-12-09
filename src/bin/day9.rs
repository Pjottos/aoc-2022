use aoc_2022::*;

use std::collections::HashSet;

fn main() {
    Harness::begin()
        .day(9)
        .extract(|text| {
            text.lines().map(|line| {
                let (dir, n) = line.split_once(' ').unwrap();
                let n = n.parse::<u8>().unwrap();
                match dir.as_bytes()[0] {
                    b'D' => (1, -1i32, n),
                    b'U' => (1, 1, n),
                    b'L' => (0, -1, n),
                    b'R' => (0, 1, n),
                    _ => unreachable!(),
                }
            })
        })
        .run_part(1, |deltas| run(deltas, &mut [[0i32; 2]; 2]))
        .run_part(2, |deltas| run(deltas, &mut [[0i32; 2]; 10]));
}

fn run(deltas: impl Iterator<Item = (i32, i32, u8)>, chain: &mut [[i32; 2]]) -> usize {
    let mut tail_positions = HashSet::new();
    for (axis, signum, amount) in deltas {
        for _ in 0..amount {
            chain[0][0] += (axis ^ 1) * signum;
            chain[0][1] += axis * signum;
            for i in 0..chain.len() - 1 {
                let move_x = chain[i][0] - chain[i + 1][0];
                let move_y = chain[i][1] - chain[i + 1][1];
                if move_x.abs() > 1 || move_y.abs() > 1 {
                    chain[i + 1][0] += move_x.signum();
                    chain[i + 1][1] += move_y.signum();
                } else {
                    break;
                }
            }
            tail_positions.insert(chain.last().copied().unwrap());
        }
    }
    tail_positions.len()
}
