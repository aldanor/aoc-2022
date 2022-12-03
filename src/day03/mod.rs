#[allow(unused_imports)]
use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn char_to_score(c: u8) -> u8 {
    if c >= b'a' {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    }
}

pub fn part1(mut s: &[u8]) -> u32 {
    let mut total = 0;
    'outer: while s.len() > 1 {
        let n = s.memchr(b'\n');
        let line = &s[..n];
        s = &s[(n + 1)..];
        let (line_r, line_l) = line.split_at(n >> 1);
        let (mut left, mut right) = (0u64, 0u64);
        for i in 0..(n >> 1) {
            left |= 1u64 << (line_l[i] - 64); // 64 is faster (on ARM, shifts are 6-bit masked)
            right |= 1u64 << (line_r[i] - 64);
            let cross = left & right;
            if cross != 0 {
                total += char_to_score(cross.trailing_zeros() as u8 | 64) as u32;
                continue 'outer;
            }
        }
    }
    total
}

pub fn part2(mut s: &[u8]) -> u32 {
    let mut total = 0;
    while s.len() > 1 {
        let mut badge = u64::MAX;
        for _ in 0..3 {
            let n = s.memchr(b'\n');
            badge &= s[..n].iter().fold(0u64, |acc, &c| acc | (1 << (c - 64)));
            s = &s[(n + 1)..];
        }
        total += char_to_score(badge.trailing_zeros() as u8 | 64) as u32;
    }
    total
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 8349);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 2681);
}
