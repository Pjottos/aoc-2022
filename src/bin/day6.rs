use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(6)
        .extract(|text| text.as_bytes())
        .run_part(1, |bytes| {
            bytes
                .windows(4)
                .position(|window| (0..4).all(|i| window[i + 1..].iter().all(|&w| w != window[i])))
                .unwrap()
                + 4
        })
        .run_part(2, |bytes| {
            bytes
                .windows(14)
                .position(|window| (0..14).all(|i| window[i + 1..].iter().all(|&w| w != window[i])))
                .unwrap()
                + 14
        });
}
