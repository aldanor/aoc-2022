use std::iter;
use std::ptr;

use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_4digit(a: [u8; 4]) -> u32 {
    let mut v: u32 = bytemuck::cast(a);
    v -= 0x30303030;
    v = (v * 10) + (v >> 8);
    (v & 0xff) * 100 + ((v & 0xff0000) >> 16)
}

fn parse_number(s: &mut &[u8]) -> u32 {
    let mut v = unsafe { parse_4digit(ptr::read_unaligned(s.as_ptr().cast::<[u8; 4]>())) };
    *s = s.advance(4);
    loop {
        let d = s.get_at(0);
        if d & 0b10000 != 0 {
            v = v * 10 + u32::from(d - b'0');
            *s = s.advance(1);
        } else {
            break;
        }
    }
    *s = s.advance(1);
    v
}

pub fn stacks(mut s: &[u8]) -> impl Iterator<Item = u32> + '_ {
    iter::from_fn(move || {
        if s.len() > 1 {
            let mut total = 0;
            loop {
                total += parse_number(&mut s);
                if s.get_at(0) == b'\n' {
                    s = s.advance(1);
                    return Some(total);
                } else if s.len() <= 1 {
                    return Some(total);
                }
            }
        } else {
            None
        }
    })
}

fn acc_maxn<T: Integer, const N: usize>() -> impl Fn([T; N], T) -> [T; N] {
    move |mut max, x| {
        for i in 0..N {
            if x > max[i] {
                for j in (i + 1..N).rev() {
                    max[j] = max[j - 1];
                }
                max[i] = x;
                break;
            }
        }
        max
    }
}

pub fn part1(s: &[u8]) -> u32 {
    stacks(s).max().unwrap_or(0)
}

pub fn part2(s: &[u8]) -> u32 {
    stacks(s).fold([0; 3], acc_maxn::<u32, 3>()).into_iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 70509);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 208567);
}
