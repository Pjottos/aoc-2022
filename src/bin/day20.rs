#![feature(portable_simd)]
#![feature(new_uninit)]

use std::{
    arch::x86_64::*,
    cmp::Ordering,
    simd::{u16x16, SimdPartialEq},
};

use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(20)
        .extract(|text| {
            let mut nums = Vec::with_capacity(5000);
            let mut idxs = Vec::with_capacity(5000);
            let mut i = 0u16;
            let mut negative = false;
            let mut num = 0i16;
            for &b in text.as_bytes() {
                match b {
                    b'\n' => {
                        nums.push(if negative { -num } else { num });
                        idxs.push(i);
                        i = i.checked_add(1).unwrap();
                        negative = false;
                        num = 0;
                    }
                    b'-' => negative = true,
                    b => num = num * 10 + i16::from(b - b'0'),
                }
            }

            (nums.into_boxed_slice(), idxs.into_boxed_slice())
        })
        .run_part(1, |(mut nums, mut idxs)| {
            mix_nums(&mut nums, &mut idxs);

            let zero_idx = nums.iter().position(|&n| n == 0).unwrap() as u16;
            (1..=3)
                .map(|i| nums[((zero_idx + (i * 1000)) % (nums.len() as u16)) as usize] as i32)
                .sum::<i32>()
        })
        .run_part(2, |(mut nums, mut idxs)| {
            const KEY: i64 = 811589153;
            let divisor = nums.len() as i64 - 1;
            let real_nums = nums
                .iter_mut()
                .map(|n| {
                    let multiplied = i64::from(*n) * KEY;
                    *n = (multiplied % divisor) as i16;
                    multiplied
                })
                .collect::<Box<_>>();

            for _ in 0..10 {
                mix_nums(&mut nums, &mut idxs);
            }

            let zero_idx = nums.iter().position(|&n| n == 0).unwrap() as u16;
            (1..=3)
                .map(|i| {
                    let nums_idx = (zero_idx + (i * 1000)) % nums.len() as u16;
                    let real_idx = idxs[nums_idx as usize];
                    real_nums[real_idx as usize]
                })
                .sum::<i64>()
        });
}

fn mix_nums(nums: &mut [i16], idxs: &mut [u16]) {
    assert_eq!(nums.len(), idxs.len());
    let num_count = nums.len() as u16;
    for i in 0..num_count {
        let (pre, chunks, post) = idxs.as_simd::<16>();
        let idx = pre
            .iter()
            .copied()
            .position(|real_idx| real_idx == i)
            .or_else(|| {
                chunks.iter().enumerate().find_map(|(c, chunk)| unsafe {
                    let mask = chunk.simd_eq(u16x16::splat(i));
                    let bit_mask = _mm256_movemask_epi8(mask.to_int().into());
                    (bit_mask != 0)
                        .then(|| pre.len() + c * 16 + (bit_mask.trailing_zeros() / 2) as usize)
                })
            })
            .or_else(|| {
                post.iter()
                    .copied()
                    .position(|real_idx| real_idx == i)
                    .map(|p| pre.len() + chunks.len() * 16 + p)
            })
            .unwrap() as u16;
        let num = nums[idx as usize];

        let mut new_idx = (idx as i16 + num).rem_euclid(num_count as i16 - 1) as u16;
        if new_idx == idx {
            continue;
        }
        if new_idx == 0 {
            new_idx = num_count - 1;
        }

        match new_idx.cmp(&idx) {
            Ordering::Less => {
                nums[new_idx as usize..idx as usize + 1].rotate_right(1);
                idxs[new_idx as usize..idx as usize + 1].rotate_right(1);
            }
            Ordering::Greater => {
                nums[idx as usize..new_idx as usize + 1].rotate_left(1);
                idxs[idx as usize..new_idx as usize + 1].rotate_left(1);
            }
            Ordering::Equal => (),
        }
        nums[new_idx as usize] = num;
        idxs[new_idx as usize] = i;
    }
}
