use aoc_2022::*;

use std::iter;

fn main() {
    Harness::begin()
        .day(1)
        .extract(|text| {
            let mut bytes = text.as_bytes().iter().copied().rev().skip(1);
            iter::from_fn(move || {
                if bytes.len() == 0 {
                    return None;
                }

                let mut sum = 0;
                let mut next = bytes.next().unwrap();
                loop {
                    if next == b'\n' {
                        next = bytes.next().unwrap();
                        if next == b'\n' {
                            break Some(sum);
                        }
                    }
                    for multiplier in [1, 10, 100, 1000] {
                        sum += (next - b'0') as u32 * multiplier;
                        next = bytes.next().unwrap_or(0);
                    }
                    if next & 0x10 != 0 {
                        sum += (next - b'0') as u32 * 10000;
                        next = bytes.next().unwrap_or(0);
                    }
                    if next == 0 {
                        break Some(sum);
                    }
                }
            })
        })
        .run_part(1, |sums| sums.max().unwrap())
        .run_part(2, |sums| {
            let mut result = [0; 3];
            for sum in sums {
                if sum > result[0] {
                    result[2] = result[1];
                    result[1] = result[0];
                    result[0] = sum;
                } else if sum > result[1] {
                    result[2] = result[1];
                    result[1] = sum;
                } else if sum > result[2] {
                    result[2] = sum;
                }
            }
            result.iter().sum::<u32>()
        });
}
