#[allow(unused_imports)]
use crate::utils::*;

use std::array;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

pub fn part1(s: &[u8]) -> u16 {
    let n = s.memchr(b'\n');
    // note: we could use a bunch of u128s, one per row, but the code would be more complex, w/e
    let mut out = vec![0_u8; s.len()];
    for i in 1..(n - 1) {
        for (fwd, pos, dpos) in [
            (true, i * (n + 1), 1),                // LR
            (false, i * (n + 1) + n - 1, 1),       // RL
            (true, i, n + 1),                      // UD
            (false, i + (n - 1) * (n + 1), n + 1), // DU
        ] {
            let (mut pos, mut max) = (pos, s.get_at(pos));
            for _ in 1..n as u8 {
                pos = if fwd { pos + dpos } else { pos - dpos };
                let c = s.get_at(pos);
                if c > max {
                    max = c;
                    out[pos] = 1;
                }
            }
        }
    }
    out.iter().map(|&x| x as u16).sum::<u16>() + n as u16 * 4 - 4
}

pub fn part2(s: &[u8]) -> u32 {
    let masks: [(u128, u128); 16] = array::from_fn(|i| {
        (
            u128::from_ne_bytes(array::from_fn(|j| u8::from(j <= i))),
            u128::from_ne_bytes(array::from_fn(|j| u8::from(j > i) * 0xff)),
        )
    });
    let n = s.memchr(b'\n');
    let mut out = vec![1_u32; s.len()];
    for i in 1..(n - 1) {
        for (fwd, pos, dpos) in [
            (true, i * (n + 1), 1),                // LR
            (false, i * (n + 1) + n - 1, 1),       // RL
            (true, i, n + 1),                      // UD
            (false, i + (n - 1) * (n + 1), n + 1), // DU
        ] {
            let (mut prev, mut pos) = ([0_u8; 16], pos);
            out[pos] = 0;
            for j in 1..n as u8 {
                pos = if fwd { pos + dpos } else { pos - dpos };
                let c = s.get_at(pos as usize) - b'0';
                out[pos as usize] *= j.wrapping_sub(prev[c as usize]) as u32;
                let (m1, m2) = masks[c as usize];
                prev = ((m1 * (j as u128)) | (u128::from_ne_bytes(prev) & m2)).to_ne_bytes();
            }
        }
    }
    out.iter().copied().max().unwrap_or(0)
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 1870);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 517440);
}
