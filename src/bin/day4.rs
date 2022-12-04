use aoc_2022::*;

fn main() {
    Harness::begin()
        .day(4)
        .extract(|text| text.lines().map(parse_ids))
        .run_part(1, |lines| {
            //let mut chunks = text.lines().array_chunks::<32>();

            //let counts = chunks
            //    .by_ref()
            //    .map(|chunk| {
            //        let mut vectors = [u8x32::default(); 4];
            //        for (j, line) in chunk.into_iter().enumerate() {
            //            let ids = parse_ids(line);
            //            for i in 0..4 {
            //                vectors[i].as_mut_array()[j] = ids[i];
            //            }
            //        }
            //        vectors
            //    })
            //    .fold(u8x32::default(), |counts, v| {
            //        let mask = (v[0].simd_ge(v[2]) & v[1].simd_le(v[3]))
            //            | (v[2].simd_ge(v[0]) & v[3].simd_le(v[1]));
            //        counts + (mask.to_int().cast() & u8x32::splat(1))
            //    });

            //let mut count = counts
            //    .as_array()
            //    .iter()
            //    .fold(0, |count, &c| count + c as u32);

            //count += chunks
            //    .into_remainder()
            //    .into_iter()
            //    .flat_map(|lines| lines.map(parse_ids))

            lines
                .map(|v| ((v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1])) as u32)
                .sum::<u32>()
        })
        .run_part(2, |lines| {
            lines
                .map(|v| (v[0] <= v[3] && v[1] >= v[2]) as u32)
                .sum::<u32>()
        });
}

#[inline]
fn parse_ids(line: &str) -> [u8; 4] {
    let mut bytes = line.as_bytes().iter().copied().rev();
    let mut res = [0; 4];

    for i in (0..4).rev() {
        // Parse at most 2 digits correctly
        let mut multiplier = 1;
        let mut num = 0;
        loop {
            match bytes.next() {
                Some(byte) => {
                    if byte < b'0' {
                        break;
                    } else {
                        num += (byte - b'0') * multiplier;
                        multiplier = 10;
                    }
                }
                None => break,
            }
        }
        res[i] = num;
    }

    res
}
