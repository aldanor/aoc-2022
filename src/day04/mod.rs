use std::iter;

use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_line(s: &[u8]) -> (usize, [u8; 4]) {
    let mut out = [0; 4];
    let mut pos = 0;
    for i in 0..4 {
        (out[i], pos) = (s.get_at(pos) - b'0', pos + 1);
        if s.get_at(pos) >= b'0' {
            (out[i], pos) = (out[i] * 10 + s.get_at(pos) - b'0', pos + 1);
        }
        pos += 1;
    }
    (pos, out)
}

fn iter_lines(mut s: &[u8]) -> impl Iterator<Item = [u8; 4]> + '_ {
    iter::from_fn(move || {
        if s.len() > 1 {
            let (n, r) = parse_line(s);
            s = s.advance(n);
            Some(r)
        } else {
            None
        }
    })
}

pub fn part1(s: &[u8]) -> u16 {
    iter_lines(s).map(|r| ((r[0].cmp(&r[2]) as i8) * (r[1].cmp(&r[3]) as i8) <= 0) as u16).sum()
}

pub fn part2(s: &[u8]) -> u16 {
    iter_lines(s).map(|r| (r[0] <= r[3] && r[2] <= r[1]) as u16).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 530);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 903);
}
