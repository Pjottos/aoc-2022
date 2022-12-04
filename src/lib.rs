mod harness;

pub use harness::Harness;

#[inline]
pub fn fast_u32_parse(text: &str) -> u32 {
    // Numbers with at most 8 decimal digits are guaranteed to fit in a u32
    const POWERS_OF_10: [u32; 9] = [
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ];
    text.as_bytes()
        .iter()
        .rev()
        .take(8)
        .map(|b| b - b'0')
        .enumerate()
        .fold(0, |sum, (i, digit)| sum + digit as u32 * POWERS_OF_10[i])
}
