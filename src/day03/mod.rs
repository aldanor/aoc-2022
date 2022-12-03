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
    while s.len() > 1 {
        let n = s.memchr(b'\n');
        let line = &s[..n];
        let left = line[..n >> 1].iter().fold(0u64, |acc, &c| acc | (1 << (c - b'A')));
        let right = line[n >> 1..].iter().fold(0u64, |acc, &c| acc | (1 << (c - b'A')));
        total += char_to_score((left & right).trailing_zeros() as u8 + b'A') as u32;
        s = &s[(n + 1)..];
    }
    total
}

pub fn part2(mut s: &[u8]) -> u32 {
    let mut total = 0;
    while s.len() > 1 {
        let mut badge = u64::MAX;
        for _ in 0..3 {
            let n = s.memchr(b'\n');
            badge &= s[..n].iter().fold(0u64, |acc, &c| acc | (1 << (c - b'A')));
            s = &s[(n + 1)..];
        }
        total += char_to_score(badge.trailing_zeros() as u8 + b'A') as u32;
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
