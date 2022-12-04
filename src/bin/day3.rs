#![feature(iter_array_chunks)]

use aoc_2022::*;

use std::ops::BitAnd;

fn main() {
    Harness::begin()
        .day(3)
        .extract(|text| text.lines().map(str::as_bytes))
        .run_part(1, |rucksacks| {
            rucksacks
                .clone()
                .map(|rucksack| {
                    let (comp_a, comp_b) = rucksack.split_at(rucksack.len() / 2);
                    let set_a = comp_a.iter().copied().fold(0, priority_set_fold);
                    let set_b = comp_b.iter().copied().fold(0, priority_set_fold);
                    (set_a & set_b).trailing_zeros()
                })
                .sum::<u32>()
        })
        .run_part(2, |rucksacks| {
            rucksacks
                .clone()
                .array_chunks::<3>()
                .map(|group| {
                    group
                        .iter()
                        .map(|rucksack| rucksack.iter().copied().fold(0, priority_set_fold))
                        .fold(!0, u64::bitand)
                        .trailing_zeros()
                })
                .sum::<u32>()
        });
}

#[inline]
fn priority_set_fold(set: u64, item: u8) -> u64 {
    let priority = if item & 0x20 != 0 {
        item - 0x60
    } else {
        item - (0x40 - 26)
    };
    set | (1 << priority)
}
