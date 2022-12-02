use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(2)
        .extract(|text| {
            text.lines().map(|l| {
                let b = l.as_bytes();
                (b[0] - b'A', b[2] - b'X')
            })
        })
        .run_part(1, |rounds| {
            rounds.clone().fold(0, |score, round| {
                let shape_score = round.1 as u32 + 1;
                let outcome_score = ((round.1 + 4 - round.0) % 3) * 3;
                score + shape_score + outcome_score as u32
            })
        })
        .run_part(2, |rounds| {
            rounds.clone().fold(0, |score, round| {
                let play_to_make = (round.0 + 2 + round.1) % 3;
                let shape_score = play_to_make as u32 + 1;
                let outcome_score = (round.1 * 3) as u32;
                score + shape_score + outcome_score
            })
        });
}
