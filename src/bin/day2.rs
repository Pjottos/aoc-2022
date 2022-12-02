#![feature(portable_simd)]
#![feature(array_chunks)]

use aoc_2022::*;

use std::simd::u16x16;

fn main() {
    Harness::begin()
        .day(2)
        .extract(|text| {
            // Assume little endian
            let slice = bytemuck::cast_slice(text.as_bytes());
            let (pre, chunks, post) = slice.as_simd::<16>();

            let chunks = chunks.array_chunks::<2>().map(|&[a, b]| {
                let (mut a, mut b) = a.deinterleave(b);
                a -= u16x16::splat(b'A' as u16 | ((b' ' as u16) << 8));
                b -= u16x16::splat(b'X' as u16 | ((b'\n' as u16) << 8));
                (a, b)
            });

            let scalar_map = |round: &[u16; 2]| {
                let a = round[0] - (b'A' as u16 | ((b' ' as u16) << 8));
                let b = round[1] - (b'X' as u16 | ((b'\n' as u16) << 8));

                (a, b)
            };
            let pre = pre.array_chunks::<2>().map(scalar_map);
            let post = post.array_chunks::<2>().map(scalar_map);

            (pre, chunks, post)
        })
        .run_part(1, |extracted| {
            let (pre, chunks, post) = extracted;
            let scalar_fold_func = |score, (a, b): (u16, u16)| {
                let play_score = b + 1;
                let outcome_score = ((b + 4 - a) % 3) * 3;
                score + play_score + outcome_score
            };

            let mut score = pre.clone().fold(0, scalar_fold_func);

            let mut score_vector = u16x16::default();
            for (a, b) in chunks.clone() {
                let play_score = b + u16x16::splat(1);
                let outcome_score =
                    ((b + u16x16::splat(4) - a) % u16x16::splat(3)) * u16x16::splat(3);
                score_vector += play_score + outcome_score;
            }
            score += score_vector.as_array().iter().sum::<u16>();

            score = post.clone().fold(score, scalar_fold_func);

            score
        })
        .run_part(2, |extracted| {
            let (pre, chunks, post) = extracted;
            let scalar_fold_func = |score, (a, b): (u16, u16)| {
                let play_to_make = (a + 2 + b) % 3;
                let play_score = play_to_make + 1;
                let outcome_score = b * 3;
                score + play_score + outcome_score
            };

            let mut score = pre.clone().fold(0, scalar_fold_func);

            let mut score_vector = u16x16::default();
            for (a, b) in chunks.clone() {
                let play_to_make = (a + u16x16::splat(2) + b) % u16x16::splat(3);
                let play_score = play_to_make + u16x16::splat(1);
                let outcome_score = b * u16x16::splat(3);
                score_vector += play_score + outcome_score;
            }
            score += score_vector.as_array().iter().sum::<u16>();

            score = post.clone().fold(score, scalar_fold_func);

            score
        });
}
