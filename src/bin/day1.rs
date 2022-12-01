use aoc_2022::Harness;

fn main() {
    Harness::begin()
        .day(1)
        .extract(|text| {
            text.split("\n\n").map(|block| {
                block
                    .lines()
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
        })
        .run_part(1, |sums| sums.clone().max().unwrap())
        .run_part(2, |sums| {
            let mut sums = sums.clone().collect::<Vec<_>>();
            sums.sort_unstable();
            sums.iter().rev().take(3).sum::<u32>()
        });
}
