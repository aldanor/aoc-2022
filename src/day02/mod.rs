#[allow(unused_imports)]
use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes_align_as!(u32, "input.txt")
}

#[allow(dead_code)]
fn find_mapping() {
    // we can run this once to generate the magic numbers and it will work for all inputs
    let s = bytemuck::cast_slice::<_, u32>(include_bytes_align_as!(u32, "9.txt"));
    let mut r = rand::thread_rng();
    'outer: loop {
        let mul: u32 = rand::Rng::gen(&mut r);
        let shift = 27; // 32 - 27 = 5 bits left, 2^5 = 32 for shifts around the u32
        let mut x = [0_u8; 9];
        for i in 0..9 {
            x[i] = (s[i].wrapping_mul(mul) >> shift) as u8;
        }
        // we want to ensure no collisions between 3-bit values so we can pack the score map
        for i in 0..8 {
            for j in (i + 1)..9 {
                if ((x[i] as i8) - (x[j] as i8)).abs() < 3 {
                    continue 'outer;
                }
            }
        }
        // now we can pack all scores (sub 1 from all, and 2 from 9) into the u32 at given indices
        let build_map = |table: [u8; 9], x: [u8; 9]| {
            (0..9).fold(0_u32, |a, i| a | ((table[i] - 1).min(7) as u32) << x[i])
        };
        let map1 = build_map([4, 8, 3, 1, 5, 9, 7, 2, 6], x);
        let map2 = build_map([3, 4, 8, 1, 5, 9, 2, 6, 7], x);
        println!("mul:   {:#08x}", mul);
        println!("shift: {}", shift);
        println!("idx:   {:?}", x);
        println!("'9':   {:#08x}", s[5]);
        println!("map1:  {:#08x}", map1);
        println!("map2:  {:#08x}", map2);
        break;
    }
}

pub fn solve(s: &[u8], score_map: u32) -> u32 {
    /*
    mul:   0x58bb8d84
    shift: 27
    idx:   [24, 10, 28, 3, 21, 7, 15, 0, 18]
    '9':   0xa5a2042
    map1:  0x23971f81
    map2:  0x72988f85
     */
    bytemuck::cast_slice::<_, u32>(s)
        .iter()
        .map(|&x| ((score_map >> (x.wrapping_mul(0x58bb8d84) >> 27)) & 7) + (x == 0xa5a2042) as u32)
        .sum::<u32>()
        + (s.len() >> 2) as u32
}

pub fn part1(s: &[u8]) -> u32 {
    solve(s, 0x23971f81)
}

pub fn part2(s: &[u8]) -> u32 {
    solve(s, 0x72988f85)
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 11873);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 12014);
}
