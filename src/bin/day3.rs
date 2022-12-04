#![feature(iter_array_chunks)]
#![feature(portable_simd)]

use aoc_2022::*;

use std::{
    iter,
    simd::{u8x32, Simd},
};

fn main() {
    Harness::begin()
        .day(3)
        .extract(|text| {
            let bytes = text.as_bytes();
            let line_ends = bytes
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(i, b)| (b == b'\n').then_some(i))
                .array_chunks::<32>();
            (bytes, line_ends)
        })
        .run_part(1, |(bytes, line_ends)| {
            let mut line_ends = line_ends.clone();
            let mut next_idx = 0;
            let vectors_iters = line_ends.by_ref().map(move |line_ends| {
                let max_len = line_ends
                    .iter()
                    .enumerate()
                    .map(|(i, end)| {
                        let start = if i == 0 {
                            next_idx
                        } else {
                            line_ends[i - 1] + 1
                        };
                        end - start
                    })
                    .max()
                    .unwrap();

                (0..max_len).map(move |j| {
                    let mut vector = u8x32::default();
                    for i in 0..32 {
                        let start = if i == 0 {
                            next_idx
                        } else {
                            line_ends[i - 1] + 1
                        };
                        let rucksack_len = line_ends[i] - start;
                        vector.as_mut_array()[i] = if j >= rucksack_len / 2 && j < max_len / 2 {
                            // Insert padding so that all rows in the chunk have the center on the
                            // same column, and they have the same total number of columns
                            0
                        } else {
                            let idx = if j >= max_len / 2 {
                                // Correct for the padding applied to align the centers
                                rucksack_len / 2 + (j - max_len / 2)
                            } else {
                                j
                            };
                            bytes[idx]
                        };
                    }

                    next_idx = line_ends[31] + 1;

                    vector
                })
            });
            for vectors in vectors_iters {
                let max_len = vectors.len();
                let mut masks = [u8x32::default(); 52];
                for vector in vectors.take(max_len / 2) {}
            }
            //rucksacks
            //    .clone()
            //    .map(|line| {
            //        let (comp_a, comp_b) = line.split_at(line.len() / 2);
            //        let item_kind = comp_a
            //            .as_bytes()
            //            .iter()
            //            .copied()
            //            .find(|a| comp_b.as_bytes().iter().any(|b| a == b))
            //            .unwrap();
            //        priority(item_kind) as u32
            //    })
            //    .sum::<u32>()
        })
        .run_part(2, |rucksacks| {
            //rucksacks
            //    .clone()
            //    .array_chunks::<3>()
            //    .map(|group| {
            //        let mut masks = [0u64; 3];
            //        for (rucksack, mask) in group.iter().zip(masks.iter_mut()) {
            //            *mask = rucksack
            //                .as_bytes()
            //                .iter()
            //                .copied()
            //                .fold(0, |mask, item_kind| mask | (1 << (priority(item_kind) - 1)));
            //        }
            //        let badge_bit = masks
            //            .into_iter()
            //            .fold(!0, |total_mask, mask| total_mask & mask);
            //        badge_bit.trailing_zeros() + 1
            //    })
            //    .sum::<u32>()
        });
}

#[inline]
fn priority(item_kind: u8) -> u8 {
    let is_lowercase_bit = item_kind & 0x20;
    (item_kind & !0x40) + ((is_lowercase_bit >> 5) ^ 1) * 26 - is_lowercase_bit
}
