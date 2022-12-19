use aoc_2022::*;

use std::{collections::VecDeque, iter};

fn main() {
    Harness::begin()
        .day(12)
        .extract(|text| {
            let mut col_count = None;
            let mut start_pos = None;
            let mut target_pos = None;

            for (i, &b) in text.as_bytes().iter().enumerate() {
                if col_count.is_none() && b == b'\n' {
                    col_count = Some(i);
                } else if start_pos.is_none() && b == b'S' {
                    start_pos = Some(i);
                } else if target_pos.is_none() && b == b'E' {
                    target_pos = Some(i);
                }
            }

            Map {
                bytes: text.as_bytes(),
                col_count: col_count.unwrap(),
                start_pos: start_pos.unwrap(),
                target_pos: target_pos.unwrap(),
            }
        })
        .run_part(1, |map| map.bfs_path::<false>())
        .run_part(2, |map| map.bfs_path::<true>());
}

struct Map<'a> {
    bytes: &'a [u8],
    col_count: usize,
    start_pos: usize,
    target_pos: usize,
}

impl<'a> Map<'a> {
    fn bfs_path<const PART2: bool>(&self) -> u32 {
        let mut seen = vec![0u64; (self.bytes.len() + 63) / 64];
        let mut visit_queue: VecDeque<_> = iter::once((self.target_pos, 0)).collect();

        let can_visit = |old, new| {
            #[inline]
            fn height(v: u8) -> u8 {
                match v {
                    b'S' => b'a',
                    b'E' => b'z',
                    v => v,
                }
            }

            height(self.bytes[old]) <= height(self.bytes[new]) + 1
        };

        while let Some((idx, length)) = visit_queue.pop_front() {
            if PART2 {
                if self.bytes[idx] == b'a' || self.bytes[idx] == b'S' {
                    return length;
                }
            } else if idx == self.start_pos {
                return length;
            }

            //self.render(&seen, idx);
            //println!();

            let x = idx % (self.col_count + 1);
            visit_queue.extend(
                (x != 0)
                    .then_some(idx.wrapping_sub(1))
                    .into_iter()
                    .chain((x != self.col_count - 1).then_some(idx + 1))
                    .chain(idx.checked_sub(self.col_count + 1))
                    .chain(
                        (idx < self.bytes.len() - (self.col_count + 1))
                            .then_some(idx + (self.col_count + 1)),
                    )
                    .filter_map(|i| {
                        (can_visit(idx, i) && seen[i / 64] & (1 << (i % 64)) == 0).then(|| {
                            seen[i / 64] |= 1 << (i % 64);
                            (i, length + 1)
                        })
                    }),
            );
        }

        panic!("no path")
    }

    #[allow(dead_code)]
    fn render(&self, seen: &[u64], cur_idx: usize) {
        for (i, b) in self.bytes.iter().copied().enumerate() {
            let c = if i == cur_idx {
                '█'
            } else if seen[i / 64] & (1 << (i % 64)) != 0 {
                '□'
            } else {
                b as char
            };

            print!("{c}");
        }
    }
}
