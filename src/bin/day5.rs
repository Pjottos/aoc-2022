#![feature(iter_array_chunks)]

use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(5)
        .extract(|text| {
            let (raw_stacks, raw_instructions) = text.split_once("\nm").unwrap();

            let raw_stacks = raw_stacks.as_bytes();

            let stack_count = 1 + raw_stacks
                .iter()
                .skip(3)
                .step_by(4)
                .position(|&c| c == b'\n')
                .unwrap();
            let mut stacks = vec![vec![]; stack_count];
            for line in raw_stacks.rchunks_exact(stack_count * 4).skip(1) {
                for i in 0..stack_count {
                    let value = line[1 + i * 4];
                    if value != b' ' {
                        stacks[i].push(value);
                    }
                }
            }

            let instructions = raw_instructions
                .lines()
                .flat_map(|line| line.split(' ').flat_map(str::parse::<usize>))
                .array_chunks::<3>();

            (stacks, instructions)
        })
        .run_part(1, |(mut stacks, instructions)| {
            for [count, from, to] in instructions {
                // Thanks, borrow checker
                let first_idx = (from - 1).min(to - 1);
                let second_idx = (from - 1).max(to - 1);
                let offset = second_idx - first_idx;
                assert_ne!(offset, 0);
                let mut stacks_iter = stacks.iter_mut();
                let mut from_stack = stacks_iter.nth(first_idx).unwrap();
                let mut to_stack = stacks_iter.nth(offset - 1).unwrap();
                if first_idx != from - 1 {
                    std::mem::swap(&mut from_stack, &mut to_stack);
                }

                let idx = from_stack.len() - count;
                to_stack.extend(from_stack.drain(idx..).rev());
            }

            let tops = stacks
                .iter()
                .map(|stack| stack.last().copied().unwrap_or(b' '))
                .collect::<Vec<_>>();
            String::from_utf8(tops).unwrap()
        })
        .run_part(2, |(mut stacks, instructions)| {
            for [count, from, to] in instructions {
                // Thanks, borrow checker
                let first_idx = (from - 1).min(to - 1);
                let second_idx = (from - 1).max(to - 1);
                let offset = second_idx - first_idx;
                assert_ne!(offset, 0);
                let mut stacks_iter = stacks.iter_mut();
                let mut from_stack = stacks_iter.nth(first_idx).unwrap();
                let mut to_stack = stacks_iter.nth(offset - 1).unwrap();
                if first_idx != from - 1 {
                    std::mem::swap(&mut from_stack, &mut to_stack);
                }

                let idx = from_stack.len() - count;
                to_stack.extend(from_stack.drain(idx..));
            }

            let tops = stacks
                .iter()
                .map(|stack| stack.last().copied().unwrap_or(b' '))
                .collect::<Vec<_>>();
            String::from_utf8(tops).unwrap()
        });
}
