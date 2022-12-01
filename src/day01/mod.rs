use std::iter;

use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

pub fn stacks(mut s: &[u8]) -> impl Iterator<Item = u32> + '_ {
    iter::from_fn(move || {
        if s.len() > 1 {
            let mut total = 0;
            loop {
                total += parse_int_fast::<u32, 1, 5>(&mut s);
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
