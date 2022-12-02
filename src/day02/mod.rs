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
        let table1 = [4, 8, 3, 1, 5, 9, 7, 2, 6];
        let table2 = [3, 4, 8, 1, 5, 9, 2, 6, 7];
        let (mut map1, mut map2) = (0_u32, 0_u32);
        for i in 0..9 {
            map1 |= (table1[i]) << x[i];
            map2 |= (table2[i]) << x[i];
        }
        // there WILL be overlaps and collisions, but let's see if we can find when there's none
        for i in 0..9 {
            if table1[i] != (map1 >> x[i]) & 0xf || table2[i] != (map2 >> x[i]) & 0xf {
                continue 'outer;
            }
        }
        println!("mul:   {:#08x}", mul);
        println!("shift: {}", shift);
        println!("map1:  {:#08x}", map1);
        println!("map2:  {:#08x}", map2);
        println!("idx:   {:?}", x);
        break;
    }
}

pub fn solve(s: &[u8], score_map: u32) -> u32 {
    /*
    mul:   0x54d672f3
    shift: 27
    map1:  0xd024a357
    map2:  0xe8259852
    idx:   [11, 25, 8, 21, 4, 18, 0, 14, 29]
     */
    bytemuck::cast_slice::<_, u32>(s)
        .iter()
        .map(|&x| ((score_map >> (x.wrapping_mul(0x54d672f3) >> 27)) & 15))
        .sum()
}

pub fn part1(s: &[u8]) -> u32 {
    solve(s, 0xd024a357)
}

pub fn part2(s: &[u8]) -> u32 {
    solve(s, 0xe8259852)
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 11873);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 12014);
}
