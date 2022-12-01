use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(1)
        .extract(|text| {
            text.split("\n\n")
                .map(|block| block.lines().map(fast_u32_parse).sum::<u32>())
        })
        .run_part(1, |sums| sums.clone().max().unwrap())
        .run_part(2, |sums| {
            let mut sums = sums.clone().collect::<Vec<_>>();
            sums.sort_unstable();
            sums.iter().rev().take(3).sum::<u32>()
        });
}
