#![feature(portable_simd)]

use aoc_2022::*;

use std::{
    iter,
    simd::{u8x32, SimdPartialOrd},
};

fn main() {
    Harness::begin()
        .day(4)
        .extract(|text| {
            RangeVectors {
                // Skip the newline at the end of the file
                bytes: text.as_bytes().iter().copied().rev().skip(1),
            }
        })
        .run_part(1, |mut vectors| {
            let mut count = vectors
                .by_ref()
                .fold(u8x32::default(), |counts, v| {
                    let mask = (v[0].simd_ge(v[2]) & v[1].simd_le(v[3]))
                        | (v[2].simd_ge(v[0]) & v[3].simd_le(v[1]));
                    counts + (mask.to_int().cast() & u8x32::splat(1))
                })
                .as_array()
                .iter()
                .fold(0, |count, &c| count + c as u32);

            count += vectors
                .into_remainder()
                .map(|v| ((v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1])) as u32)
                .sum::<u32>();

            count
        })
        .run_part(2, |mut vectors| {
            let mut count = vectors
                .by_ref()
                .fold(u8x32::default(), |counts, v| {
                    let mask = v[0].simd_le(v[3]) & v[1].simd_ge(v[2]);
                    counts + (mask.to_int().cast() & u8x32::splat(1))
                })
                .as_array()
                .iter()
                .fold(0, |count, &c| count + c as u32);

            count += vectors
                .into_remainder()
                .map(|v| (v[0] <= v[3] && v[1] >= v[2]) as u32)
                .sum::<u32>();

            count
        });
}

struct RangeVectors<B> {
    bytes: B,
}

impl<B> Iterator for RangeVectors<B>
where
    B: Iterator<Item = u8> + ExactSizeIterator,
{
    type Item = [u8x32; 4];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Conservatively exit if we may not be able to fill a full chunk
        // - 1 because the iterator should currently be at the first digit of a line
        // so exclude one newline
        if self.bytes.len() < 32 * 3 * 4 - 1 {
            return None;
        }

        let mut v = [u8x32::default(); 4];
        for line in 0..32 {
            self.parse_line(|i, num| v[i][line] = num);
        }

        Some(v)
    }
}

impl<B> RangeVectors<B>
where
    B: Iterator<Item = u8> + ExactSizeIterator,
{
    #[inline]
    fn parse_line<F: FnMut(usize, u8)>(&mut self, mut num_func: F) {
        for i in (0..4).rev() {
            let mut num = self.bytes.next().unwrap() - b'0';
            // There may not be a next character
            let next = self.bytes.next().unwrap_or(0);
            // Out of the possible characters in the input, only digits have this bit set
            if next & 0x10 != 0 {
                num += (next - b'0') * 10;
                // Consume the next non-digit character, if any
                self.bytes.next();
            }

            num_func(i, num);
        }
    }

    #[inline]
    fn into_remainder(mut self) -> impl Iterator<Item = [u8; 4]> {
        iter::from_fn(move || {
            (self.bytes.len() != 0).then(|| {
                let mut res = [0; 4];
                self.parse_line(|i, num| res[i] = num);
                res
            })
        })
    }
}
