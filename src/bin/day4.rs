use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(4)
        .extract(|text| {
            text.lines().map(|line| {
                let mut ranges = line.split(',').map(|range| {
                    let (first, last) = range.split_once('-').unwrap();
                    fast_u32_parse(first)..fast_u32_parse(last) + 1
                });
                (ranges.next().unwrap(), ranges.next().unwrap())
            })
        })
        .run_part(1, |pairs| {
            pairs
                .clone()
                .filter(|(a, b)| {
                    (a.start >= b.start && a.end <= b.end) || (b.start >= a.start && b.end <= a.end)
                })
                .count()
        })
        .run_part(2, |pairs| {
            pairs
                .clone()
                .filter(|(a, b)| {
                    a.contains(&b.start)
                        || a.contains(&(b.end - 1))
                        || b.contains(&a.start)
                        || b.contains(&(a.end - 1))
                })
                .count()
        });
}
