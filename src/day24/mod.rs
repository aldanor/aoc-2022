#[allow(unused_imports)]
use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

pub fn part1(_s: &[u8]) -> u16 {
    0
}

pub fn part2(_s: &[u8]) -> u16 {
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 0);
}
