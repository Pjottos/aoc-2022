#![feature(iter_array_chunks)]

use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(3)
        .extract(|text| text.lines())
        .run_part(1, |rucksacks| {
            rucksacks
                .clone()
                .map(|line| {
                    let (comp_a, comp_b) = line.split_at(line.len() / 2);
                    let item_kind = comp_a
                        .as_bytes()
                        .iter()
                        .copied()
                        .find(|a| comp_b.as_bytes().iter().any(|b| a == b))
                        .unwrap();
                    priority(item_kind) as u32
                })
                .sum::<u32>()
        })
        .run_part(2, |rucksacks| {
            rucksacks
                .clone()
                .array_chunks::<3>()
                .map(|group| {
                    let mut masks = [0u64; 3];
                    for (rucksack, mask) in group.iter().zip(masks.iter_mut()) {
                        *mask = rucksack
                            .as_bytes()
                            .iter()
                            .copied()
                            .fold(0, |mask, item_kind| mask | (1 << (priority(item_kind) - 1)));
                    }
                    let badge_bit = masks
                        .into_iter()
                        .fold(!0, |total_mask, mask| total_mask & mask);
                    badge_bit.trailing_zeros() + 1
                })
                .sum::<u32>()
        });
}

#[inline]
fn priority(item_kind: u8) -> u8 {
    let is_lowercase_bit = item_kind & 0x20;
    (item_kind & !0x40) + ((is_lowercase_bit >> 5) ^ 1) * 26 - is_lowercase_bit
}
