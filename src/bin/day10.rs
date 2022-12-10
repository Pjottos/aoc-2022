use aoc_2022::*;

use std::fmt::{Debug, Write};

fn main() {
    Harness::begin()
        .day(10)
        .extract(|text| text.lines().map(|l| l.split(' ')))
        .run_part(1, |mut lines| {
            let mut reg_x = 1;
            let mut sum = 0;
            let mut stall_count = 1;
            let mut add_arg = 0;
            for cycle in 0..=220 {
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    sum += cycle * reg_x;
                }
                stall_count -= 1;
                if stall_count == 0 {
                    reg_x += add_arg;
                    add_arg = 0;

                    let mut parts = lines.next().unwrap();
                    let inst = parts.next().unwrap();
                    if inst == "noop" {
                        stall_count = 1;
                    } else if inst == "addx" {
                        add_arg = parts
                            .next()
                            .and_then(|p| str::parse::<i32>(p).ok())
                            .unwrap();
                        stall_count = 2;
                    }
                }
            }

            sum
        })
        .run_part(2, |mut lines| {
            let mut reg_x = 1i32;
            let mut stall_count = 1;
            let mut add_arg = 0;
            let mut image = CrtImage::new();
            for cycle in 0..40 * 6 {
                stall_count -= 1;
                if stall_count == 0 {
                    reg_x += add_arg;
                    add_arg = 0;

                    let mut parts = lines.next().unwrap();
                    let inst = parts.next().unwrap();
                    if inst == "noop" {
                        stall_count = 1;
                    } else if inst == "addx" {
                        add_arg = parts
                            .next()
                            .and_then(|p| str::parse::<i32>(p).ok())
                            .unwrap();
                        stall_count = 2;
                    }
                }
                if (cycle % 40 - reg_x).abs() < 2 {
                    image.v[cycle as usize / 40][cycle as usize % 40] = b'#';
                }
            }

            image
        });
}

struct CrtImage {
    v: [[u8; 40]; 6],
}

impl Debug for CrtImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for row in &self.v {
            f.write_str(std::str::from_utf8(row).unwrap())?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl CrtImage {
    fn new() -> Self {
        Self { v: [[b'.'; 40]; 6] }
    }
}
