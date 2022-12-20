#![feature(array_windows)]
#![feature(portable_simd)]

use aoc_2022::*;

use std::{
    arch::x86_64::*,
    simd::{i8x32, u8x32, SimdInt, SimdPartialOrd},
};

fn main() {
    Harness::begin()
        .day(6)
        .extract(|text| text.as_bytes())
        .run_part(1, |bytes| {
            bytes
                .array_windows::<4>()
                .position(|window| (0..4).all(|i| window[i + 1..].iter().all(|&w| w != window[i])))
                .unwrap()
                + 4
        })
        .run_part(2, |bytes| {
            let mut i = 0;
            'windows: loop {
                for j in (0..13).rev() {
                    for k in j + 1..14 {
                        if bytes[i + j] == bytes[i + k] {
                            i += j + 1;
                            continue 'windows;
                        }
                    }
                }

                break i + 14;
            }
        });
}

#[allow(dead_code)]
fn simd_attempt<const MARKER_LEN: i8>(bytes: &[u8]) -> usize {
    let (pre, chunks, post) = bytes.as_simd::<32>();
    assert!(pre.is_empty());
    assert!(post.is_empty());
    assert_eq!(chunks.len(), 4096 / 32);

    let max_offset = i8x32::splat(-MARKER_LEN);

    let mut chunk_offsets = [0; 32];
    for i in 0..32 {
        chunk_offsets[i] = i as i8;
    }
    let chunk_offsets = i8x32::from_array(chunk_offsets);
    let _natural_chunk_offsets = chunk_offsets + i8x32::splat(1);
    let _swapped_natural_chunk_offsets =
        unsafe { _mm256_permute2x128_si256::<1>(chunk_offsets.into(), chunk_offsets.into()) };

    let mut letter_offsets = i8x32::splat(i8::MIN);
    let _sequence_len = 0;
    for (_i, mut chunk) in chunks.iter().copied().enumerate() {
        // Map letters to [0, 25]
        chunk -= u8x32::splat(b'a');
        println!("chunk  : {:02x?}", chunk);
        // Mask for selecting one of the 2 16 entry halves for the letters
        let letter_half_select = chunk.simd_ge(u8x32::splat(0x10)).to_int();
        println!("halves : {:02x?}", letter_half_select);

        unsafe {
            // Blend the letter offsets so we can use only the lower 4 bits of the chunk values
            // to retrieve the right idx
            println!("offsets: {:02x?}", i8x32::from(letter_offsets));
            let swapped_letter_offsets =
                _mm256_permute2x128_si256::<1>(letter_offsets.into(), letter_offsets.into());
            println!("swapped: {:02x?}", i8x32::from(swapped_letter_offsets));
            // The offsets of the last occurences of the letters in the chunk
            let offsets = _mm256_blendv_epi8(
                _mm256_shuffle_epi8(letter_offsets.into(), chunk.into()),
                _mm256_shuffle_epi8(swapped_letter_offsets, chunk.into()),
                letter_half_select.into(),
            );
            println!("offsets: {:02x?}", i8x32::from(offsets));
            let relative_offsets = _mm256_add_epi8(offsets, chunk_offsets.into());
            let mask = _mm256_cmpgt_epi8(relative_offsets, max_offset.into());
            println!("mask   : {:02x?}", mask);
            // Check if there is a sequence of MARKER_LEN cleared bits anywhere in the mask
            //for j in 0..32 - MARKER_LEN as usize {
            //    if (mask >> j) & (!0 >> (32 - MARKER_LEN)) == 0 {
            //        // TODO: check order of mask
            //        return i * 32 + j;
            //    }
            //}
            //_mm256_shuffle_epi8(last_chunk_offsets.into(), blended_chunk),
            //println!("{:?}", i8x32::from(last_idxs));
            let swapped_chunk = _mm256_permute2x128_si256::<1>(chunk.into(), chunk.into());
            let sorted_chunk = _mm256_blendv_epi8(
                _mm256_shuffle_epi8(chunk.into(), chunk.into()),
                _mm256_shuffle_epi8(swapped_chunk, chunk.into()),
                letter_half_select.into(),
            );
            println!("sorted : {:02x?}", i8x32::from(sorted_chunk));
            // Unfortunately we cannot do something like the opposite of a permute
            // to write values to certain indexes.
            let new_letter_offsets = _mm256_shuffle_epi8(chunk_offsets.into(), chunk.into());
            println!("new_let: {:02x?}", i8x32::from(new_letter_offsets));
        }

        letter_offsets = letter_offsets.saturating_sub(i8x32::splat(32));
    }

    panic!("no marker found")
}
