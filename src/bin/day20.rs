#![feature(portable_simd)]
#![feature(new_uninit)]

use std::{
    arch::x86_64::*,
    simd::{u16x16, SimdPartialEq}, cmp::Ordering,
};

use aoc_2022::*;

const NUM_COUNT: u16 = 5000;

fn main() {
    Harness::begin()
        .day(20)
        .extract(|text| {
            let mut nums = unsafe { Box::<[i16; NUM_COUNT as usize]>::new_zeroed().assume_init() };
            let mut idxs = unsafe { Box::<[u16; NUM_COUNT as usize]>::new_zeroed().assume_init() };
            let mut i = 0u16;
            let mut negative = false;
            let mut num = 0i16;
            for &b in text.as_bytes() {
                match b {
                    b'\n' => {
                        nums[i as usize] = if negative { -num } else { num };
                        idxs[i as usize] = i;
                        i += 1;
                        negative = false;
                        num = 0;
                    }
                    b'-' => negative = true,
                    b => num = num * 10 + i16::from(b - b'0'),
                }
            }

            assert_eq!(i, NUM_COUNT);
            (nums, idxs)
        })
        .run_part(1, |(mut nums, mut idxs)| {
            mix_nums(&mut nums, &mut idxs);

            let zero_idx = nums.iter().position(|&n| n == 0).unwrap() as u16;
            (1..=3)
                .map(|i| nums[((zero_idx + (i * 1000)) % NUM_COUNT) as usize] as i32)
                .sum::<i32>()
        })
        .run_part(2, |(mut nums, mut idxs)| {
            const KEY: i64 = 811589153;
            let real_nums = nums
                .iter_mut()
                .map(|n| {
                    let multiplied = i64::from(*n) * KEY;
                    *n = (multiplied % (i64::from(NUM_COUNT) - 1)) as i16;
                    multiplied
                })
                .collect::<Box<_>>();

            for _ in 0..10 {
                mix_nums(&mut nums, &mut idxs);
            }

            let zero_idx = nums.iter().position(|&n| n == 0).unwrap() as u16;
            (1..=3)
                .map(|i| {
                    let nums_idx = (zero_idx + (i * 1000)) % NUM_COUNT;
                    let real_idx = idxs[nums_idx as usize];
                    real_nums[real_idx as usize]
                })
                .sum::<i64>()
        });
}

fn mix_nums(nums: &mut [i16; NUM_COUNT as usize], idxs: &mut [u16; NUM_COUNT as usize]) {
    for i in 0..NUM_COUNT {
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

        let mut new_idx = (idx as i16 + num).rem_euclid(NUM_COUNT as i16 - 1) as u16;
        if new_idx == idx {
            continue;
        }
        if new_idx == 0 {
            new_idx = NUM_COUNT - 1;
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
