use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(1)
        .extract(|text| {
            text.split("\n\n")
                .map(|block| block.lines().map(fast_u32_parse).sum::<u32>())
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
