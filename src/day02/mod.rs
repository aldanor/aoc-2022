#[allow(unused_imports)]
use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes_align_as!(u32, "input.txt")
}

/*
using mask of 0x14b6cc35 and shift of 22 for each "(A|B|C) (X|Y|Z)\n" entry,
we can wrapping-mul and take the lowest 4 bits to get the outcome index (0-8):
            part1   part2
A X => 0    4       3
A Y => 1    8       4
A Z => 2    3       8
B X => 3    1       1
B Y => 4    5       5
B Z => 5    9       9
C X => 6    7       2
C Y => 7    2       6
C Z => 8    6       7
 */

pub fn solve(s: &[u8], score_map: [u8; 9]) -> u32 {
    let score_map = score_map.iter().rev().fold(0, |acc, &x| (acc << 4) | x as u64);
    bytemuck::cast_slice::<_, u32>(s)
        .iter()
        // extract 0..=8 index, multiply by 4 to get the shift, shift right and get the lower 4 bits
        .map(|&x| ((score_map >> (((x.wrapping_mul(0x14b6cc35) >> 22) & 15) << 2)) & 15) as u32)
        .sum()
}

pub fn part1(s: &[u8]) -> u32 {
    solve(s, [4, 8, 3, 1, 5, 9, 7, 2, 6])
}

pub fn part2(s: &[u8]) -> u32 {
    solve(s, [3, 4, 8, 1, 5, 9, 2, 6, 7])
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 11873);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 12014);
}
