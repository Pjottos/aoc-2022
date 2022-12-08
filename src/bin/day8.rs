use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(8)
        .extract(|text| {
            let bytes = text.as_bytes();
            let col_count = bytes.iter().position(|&b| b == b'\n').unwrap();
            let row_count = bytes.len() / (col_count + 1);
            let row_size = col_count + 1;
            (bytes, col_count, row_count, row_size)
        })
        .run_part(1, |(bytes, col_count, row_count, row_size)| {
            // edges of the grid always visible, remove duplicate corners
            let mut count = 2 * col_count + 2 * row_count - 4;

            for y in 1..row_count - 1 {
                for x in 1..col_count - 1 {
                    let tree_height = bytes[y * row_size + x];
                    let row_visible = bytes[y * row_size..y * row_size + x]
                        .iter()
                        .all(|&b| b < tree_height)
                        || bytes[y * row_size + x + 1..(y + 1) * row_size]
                            .iter()
                            .all(|&b| b < tree_height);

                    let col_visible = bytes
                        .iter()
                        .skip(x)
                        .step_by(row_size)
                        .take(y)
                        .all(|&b| b < tree_height)
                        || bytes
                            .iter()
                            .skip(x)
                            .step_by(row_size)
                            .skip(y + 1)
                            .all(|&b| b < tree_height);

                    if row_visible || col_visible {
                        count += 1;
                        continue;
                    }
                }
            }

            count
        })
        .run_part(2, |(bytes, col_count, row_count, row_size)| {
            let mut max_score = 0;

            for y in 1..row_count - 1 {
                for x in 1..col_count - 1 {
                    let tree_height = bytes[y * row_size + x];
                    let left_distance = bytes[y * row_size..y * row_size + x]
                        .iter()
                        .rev()
                        .position(|&b| b >= tree_height)
                        .map_or(x, |p| p + 1);
                    let right_distance = bytes[y * row_size + x + 1..y * row_size + col_count]
                        .iter()
                        .position(|&b| b >= tree_height)
                        .map_or(col_count - (x + 1), |p| p + 1);

                    let up_distance = bytes
                        .iter()
                        .skip(x)
                        .step_by(row_size)
                        .take(y)
                        .rev()
                        .position(|&b| b >= tree_height)
                        .map_or(y, |p| p + 1);

                    let down_distance = bytes
                        .iter()
                        .skip(x)
                        .step_by(row_size)
                        .skip(y + 1)
                        .position(|&b| b >= tree_height)
                        .map_or(row_count - (y + 1), |p| p + 1);

                    let score = left_distance * right_distance * up_distance * down_distance;
                    max_score = score.max(max_score);
                }
            }

            max_score
        });
}
