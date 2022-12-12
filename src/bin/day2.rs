#![feature(portable_simd)]
#![feature(array_chunks)]

use aoc_2022::*;

use std::{
    arch::x86_64::*,
    simd::{u16x16, u8x16, u8x32, SimdPartialEq, SimdUint},
};

fn main() {
    Harness::begin()
        .day(2)
        .extract(|text| {
            // Assume little endian
            let slice = bytemuck::cast_slice::<_, u16>(text.as_bytes());
            let (pre, chunks, post) = slice.as_simd::<16>();
            assert!(pre.is_empty());

            let chunks = chunks.array_chunks::<4>();

            let remainder = chunks
                .remainder()
                .iter()
                .flat_map(|v| v.as_array().array_chunks::<2>())
                .chain(post.array_chunks::<2>())
                .map(|round| {
                    let a = ((round[0] << 2) & 0x0C) as u8;
                    let b = (round[1] & 0x03) as u8;

                    (a, b)
                });

            (chunks, remainder)
        })
        .run_part(1, |(chunks, remainder)| {
            let mut play_score = chunks.len() as u16 * 32;
            let mut outcome_score = 0;
            let mut play_score_vector = u8x32::default();
            let mut outcome_score_vector = u8x32::default();
            let lut = u8x16::from_array([
                0xFF, 0xFF, 0xFF, 0xFF, 1, 2, 0, 0xFF, 0, 1, 2, 0xFF, 2, 0, 1, 0xFF,
            ]);
            for (i, chunk) in chunks.enumerate() {
                // This call isn't inlined if used in a .map, which is essential for performance
                let (a, b) = pack_values(chunk);

                play_score_vector += b;
                let new_outcome_score = unsafe {
                    u8x32::from(_mm256_shuffle_epi8(
                        _mm256_broadcastsi128_si256(lut.into()),
                        (a | b).into(),
                    ))
                };
                debug_assert!(!new_outcome_score.simd_eq(u8x32::splat(0xFF)).any());

                outcome_score_vector += new_outcome_score;
                // Prevent overflow
                if i % (255 / 3) == 0 {
                    play_score += play_score_vector.cast::<u16>().reduce_sum();
                    outcome_score += outcome_score_vector.cast::<u16>().reduce_sum();
                    play_score_vector = u8x32::default();
                    outcome_score_vector = u8x32::default();
                }
            }
            play_score += play_score_vector.cast::<u16>().reduce_sum();
            outcome_score += outcome_score_vector.cast::<u16>().reduce_sum();

            for (a, b) in remainder {
                play_score += b as u16 + 1;
                outcome_score += lut[(a | b) as usize] as u16;
            }

            play_score + outcome_score * 3
        })
        .run_part(2, |(chunks, remainder)| {
            let mut play_score = 0;
            let mut outcome_score = 0;
            let mut play_score_vector = u8x32::default();
            let mut outcome_score_vector = u8x32::default();
            let lut = u8x16::from_array([
                0xFF, 0xFF, 0xFF, 0xFF, 3, 1, 2, 0xFF, 1, 2, 3, 0xFF, 2, 3, 1, 0xFF,
            ]);
            for (i, chunk) in chunks.enumerate() {
                let (a, b) = pack_values(chunk);

                let new_play_score = unsafe {
                    u8x32::from(_mm256_shuffle_epi8(
                        _mm256_broadcastsi128_si256(lut.into()),
                        (a | b).into(),
                    ))
                };
                debug_assert!(!new_play_score.simd_eq(u8x32::splat(0xFF)).any());

                play_score_vector += new_play_score;
                outcome_score_vector += b;
                // Prevent overflow
                if i % (255 / 3) == 0 {
                    play_score += play_score_vector.cast::<u16>().reduce_sum();
                    outcome_score += outcome_score_vector.cast::<u16>().reduce_sum();
                    play_score_vector = u8x32::default();
                    outcome_score_vector = u8x32::default();
                }
            }
            play_score += play_score_vector.cast::<u16>().reduce_sum();
            outcome_score += outcome_score_vector.cast::<u16>().reduce_sum();

            for (a, b) in remainder {
                play_score += lut[(a | b) as usize] as u16;
                outcome_score += b as u16;
            }

            play_score + outcome_score * 3
        });
}

#[inline(always)]
fn pack_values(chunk: &[u16x16; 4]) -> (u8x32, u8x32) {
    unsafe {
        let zero = 0x80u8 as i8;
        #[rustfmt::skip]
        let shuffle_mask_a = _mm_set_epi8(
            zero, zero, zero, zero,
            0x0E, 0x0A, 0x06, 0x02,
            zero, zero, zero, zero,
            0x0C, 0x08, 0x04, 0x00,
        );
        #[rustfmt::skip]
        let shuffle_mask_b = _mm_set_epi8(
            0x0E, 0x0A, 0x06, 0x02,
            zero, zero, zero, zero,
            0x0C, 0x08, 0x04, 0x00,
            zero, zero, zero, zero,
        );
        #[rustfmt::skip]
        let shuffle_mask_c = _mm_set_epi8(
            zero, zero, zero, zero,
            0x0C, 0x08, 0x04, 0x00,
            zero, zero, zero, zero,
            0x0E, 0x0A, 0x06, 0x02,
        );
        #[rustfmt::skip]
        let shuffle_mask_d = _mm_set_epi8(
            0x0C, 0x08, 0x04, 0x00,
            zero, zero, zero, zero,
            0x0E, 0x0A, 0x06, 0x02,
            zero, zero, zero, zero,
        );

        let pack_a = _mm256_blend_epi32::<0b10101010>(
            _mm256_shuffle_epi8(chunk[0].into(), _mm256_broadcastsi128_si256(shuffle_mask_a)),
            _mm256_shuffle_epi8(chunk[2].into(), _mm256_broadcastsi128_si256(shuffle_mask_b)),
        );
        //println!("pack_a: {:02X?}", u8x32::from(pack_a));
        let pack_b = _mm256_blend_epi32::<0b10101010>(
            _mm256_shuffle_epi8(chunk[1].into(), _mm256_broadcastsi128_si256(shuffle_mask_c)),
            _mm256_shuffle_epi8(chunk[3].into(), _mm256_broadcastsi128_si256(shuffle_mask_d)),
        );
        //println!("pack_b: {:02X?}", u8x32::from(pack_b));

        // Data layout of pack_a:
        // AAAAAAAAXXXXXXXX|AAAAAAAAXXXXXXXX
        // Data layout of pack_b:
        // XXXXXXXXAAAAAAAA|XXXXXXXXAAAAAAAA

        let mut unpack_a = _mm256_blend_epi32::<0b11001100>(pack_a, pack_b);
        //println!("upck_a: {:02X?}", u8x32::from(unpack_a));
        // Starting from 1 in bits 2 and 3
        unpack_a = _mm256_and_si256(_mm256_slli_epi16::<2>(unpack_a), _mm256_set1_epi8(0x0C));
        let mut unpack_b = _mm256_alignr_epi8::<8>(pack_b, pack_a);
        //println!("upck_b: {:02X?}", u8x32::from(unpack_b));
        // Starting from 0 in bits 0 and 1
        unpack_b = _mm256_and_si256(unpack_b, _mm256_set1_epi8(0x03));

        (u8x32::from(unpack_a), u8x32::from(unpack_b))
    }
}
