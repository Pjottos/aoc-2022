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
                    let mut mask_a = 0;
                    let mut mask_b = 0;
                    let (comp_a, comp_b) = rucksack.split_at(rucksack.len() / 2);
                    for &item_kind in comp_a {
                        mask_a |= 1u64 << priority(item_kind);
                    }
                    for &item_kind in comp_b {
                        mask_b |= 1u64 << priority(item_kind);
                    }
                    (mask_a & mask_b).trailing_zeros()
                })
                .sum::<u32>()
        })
        .run_part(2, |rucksacks| {
            rucksacks
                .clone()
                .array_chunks::<3>()
                .map(|group| {
                    let mut masks = [0; 3];
                    for (i, &rucksack) in group.iter().enumerate() {
                        for &item_kind in rucksack {
                            masks[i] |= 1u64 << priority(item_kind);
                        }
                    }
                    masks.iter().fold(!0, u64::bitand).trailing_zeros()
                })
                .sum::<u32>()
        });
}

#[inline]
fn priority(item_kind: u8) -> u8 {
    if item_kind & 0x20 != 0 {
        item_kind - 0x60
    } else {
        item_kind - (0x40 - 26)
    }
}
