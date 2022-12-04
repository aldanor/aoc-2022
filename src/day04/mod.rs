use std::iter;

use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_line(s: &mut &[u8]) -> [u8; 4] {
    let mut out = [0; 4];
    for i in 0..4 {
        let c = [s.get_at(0).saturating_sub(b'0' - 1), s.get_at(1).saturating_sub(b'0' - 1)];
        if c[1] != 0 {
            out[i] = (c[0] << 4) | c[1];
            *s = s.advance(3);
        } else {
            out[i] = c[0];
            *s = s.advance(2);
        }
    }
    out
}

fn iter_lines(mut s: &[u8]) -> impl Iterator<Item = [u8; 4]> + '_ {
    iter::from_fn(move || (s.len() > 1).then(|| parse_line(&mut s)))
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
