use aoc_2022::*;

use num::integer::Integer;

use std::{collections::VecDeque, str::FromStr};

fn main() {
    Harness::begin()
        .day(11)
        .extract(|text| {
            text.split("\n\n")
                .map(|block| {
                    let mut parts = block.lines().map(|line| {
                        let (_, v) = line.split_once(':').unwrap();
                        v
                    });

                    parts.next();
                    Monkey {
                        items: parts
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|n| n.trim().parse().unwrap())
                            .collect(),
                        operation: parts.next().and_then(|op| op.parse().ok()).unwrap(),
                        divisible_test: parts
                            .next()
                            .unwrap()
                            .rsplit(' ')
                            .next()
                            .and_then(|n| n.parse().ok())
                            .unwrap(),
                        head: parts
                            .next()
                            .unwrap()
                            .rsplit(' ')
                            .next()
                            .and_then(|n| n.parse().ok())
                            .unwrap(),
                        tail: parts
                            .next()
                            .unwrap()
                            .rsplit(' ')
                            .next()
                            .and_then(|n| n.parse().ok())
                            .unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .run_part(1, |mut monkeys| run(&mut monkeys, 20, None))
        .run_part(2, |mut monkeys| {
            let lcm = monkeys
                .iter()
                .fold(1, |lcm, monkey| lcm.lcm(&monkey.divisible_test));
            run(&mut monkeys, 10000, Some(lcm))
        });
}

fn run(monkeys: &mut [Monkey], round_count: usize, lcm: Option<u64>) -> u64 {
    let mut inspect_counts = vec![0; monkeys.len()];
    for _round in 0..round_count {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspect_counts[i] += 1;
                let mut new = match monkeys[i].operation {
                    Operation::AddSelf => item + item,
                    Operation::MulSelf => item * item,
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                };
                if let Some(lcm) = lcm {
                    new %= lcm;
                } else {
                    new /= 3;
                }
                let idx = if new % monkeys[i].divisible_test == 0 {
                    monkeys[i].head
                } else {
                    monkeys[i].tail
                };
                monkeys[idx].items.push_back(new);
            }
        }
    }
    inspect_counts.sort_unstable();
    inspect_counts.iter().rev().take(2).product::<u64>()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    AddSelf,
    MulSelf,
    Add(u64),
    Mul(u64),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, op) = s.split_once("new = old ").ok_or(())?;
        let res = match op {
            "+ old" => Self::AddSelf,
            "* old" => Self::MulSelf,
            _ if op.starts_with("+") => Self::Add(
                op.split(' ')
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .ok_or(())?,
            ),
            _ if op.starts_with("*") => Self::Mul(
                op.split(' ')
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .ok_or(())?,
            ),
            _ => return Err(()),
        };
        Ok(res)
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisible_test: u64,
    head: usize,
    tail: usize,
}
