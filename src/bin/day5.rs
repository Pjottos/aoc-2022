#![feature(inline_const)]

use aoc_2022::*;

use arrayvec::ArrayVec;

use std::iter;

type CrateStack = ArrayVec<u8, 56>;

fn main() {
    Harness::begin()
        .day(5)
        .extract(|text| {
            let bytes = text.as_bytes();

            // Starting state of the stacks seems to always be
            // - 3 stacks with 8 elements
            // - 2 stacks with 7 elements
            // - 1 stack with 6 elements
            // - 1 stack with 5 elements
            // - 1 stack with 4 elements
            // - 1 stack with 3 elements
            // So the maximum amount of items in a stack is 56, and there are 9 stacks
            let raw_stacks = &bytes[..4 * 9 * 8];
            let mut stacks = [const { CrateStack::new_const() }; 9];
            for line in raw_stacks.rchunks_exact(stacks.len() * 4) {
                for i in 0..stacks.len() {
                    let value = line[1 + i * 4];
                    if value != b' ' {
                        stacks[i].push(value);
                    }
                }
            }

            let raw_instructions = &bytes[4 * 9 * 9 + 1..];
            let short_line_size = 19;
            let long_line_size = 20;
            let mut idx = 0;
            let instructions = iter::from_fn(move || {
                let is_short = *raw_instructions.get(idx + short_line_size - 1)? == b'\n';
                let line_size = if is_short {
                    short_line_size
                } else {
                    long_line_size
                };
                let b = &raw_instructions[idx..idx + line_size];
                idx += line_size;

                let instruction = if is_short {
                    [b[5] - b'0', b[12] - b'1', b[17] - b'1']
                } else {
                    [(b[5] - b'0') * 10 + b[6] - b'0', b[13] - b'1', b[18] - b'1']
                };

                Some(instruction)
            });

            (stacks, instructions)
        })
        .run_part(1, |(stacks, instructions)| {
            apply_instructions(stacks, instructions, |to, from, idx| {
                to.extend(from.drain(idx..).rev())
            })
        })
        .run_part(2, |(stacks, instructions)| {
            apply_instructions(stacks, instructions, |to, from, idx| {
                to.extend(from.drain(idx..))
            })
        });
}

#[inline]
fn apply_instructions<F>(
    mut stacks: [ArrayVec<u8, 56>; 9],
    instructions: impl Iterator<Item = [u8; 3]>,
    transfer_func: F,
) -> String
where
    F: Fn(&mut CrateStack, &mut CrateStack, usize),
{
    for [count, from, to] in instructions {
        assert_ne!(from, to);
        assert!(from < stacks.len() as u8 && to < stacks.len() as u8);
        let from_stack = unsafe { &mut *stacks.as_mut_ptr().add(from as usize) };
        let to_stack = unsafe { &mut *stacks.as_mut_ptr().add(to as usize) };
        let idx = from_stack.len() - count as usize;
        transfer_func(to_stack, from_stack, idx);
    }

    let tops = stacks
        .iter()
        .map(|stack| stack.last().copied().unwrap_or(b' '))
        .collect::<Vec<_>>();
    String::from_utf8(tops).unwrap()
}
