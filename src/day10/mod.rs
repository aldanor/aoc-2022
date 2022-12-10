use std::iter;

use bstr::BString;

use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_input(mut s: &[u8]) -> impl Iterator<Item = Option<i16>> + '_ {
    iter::from_fn(move || {
        (!s.is_empty()).then(|| {
            if s.get_at(0) == b'a' {
                s = s.advance(5);
                Some(parse_int_fast_signed::<i16, 1, 2>(&mut s))
            } else {
                s = s.advance(5);
                None
            }
        })
    })
}

pub fn part1(s: &[u8]) -> i16 {
    let (mut x, mut out) = (1, 0);
    let (mut cycle, mut next) = (1, 20);
    for cmd in parse_input(s) {
        cycle += 1 + cmd.is_some() as i16;
        if cycle > next {
            out += x * next;
            next += 40;
        }
        x += cmd.unwrap_or(0);
    }
    out
}

const W: usize = 40;
const H: usize = 6;

pub fn part2(s: &[u8]) -> BString {
    let mut out = vec![b'\n'; W * H + H];
    let (mut x, mut cycle) = (1i16, 0_usize);
    let pixel = |x: i16, cycle| b'#' + (b'.' - b'#') * (x.abs_diff(cycle as i16 % 40) > 1) as u8;
    for cmd in parse_input(s) {
        match cmd {
            Some(c) => {
                out[cycle + 1 + cycle / W] = pixel(x, cycle);
                cycle += 1;
                out[cycle + 1 + cycle / W] = pixel(x, cycle);
                cycle += 1;
                x += c;
            }
            None => {
                out[cycle + 1 + cycle / W] = pixel(x, cycle);
                cycle += 1;
            }
        }
    }
    out.into()
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 16880);
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(input()),
        BString::new(
            br"
###..#..#..##..####..##....##.###..###..
#..#.#.#..#..#....#.#..#....#.#..#.#..#.
#..#.##...#..#...#..#..#....#.###..#..#.
###..#.#..####..#...####....#.#..#.###..
#.#..#.#..#..#.#....#..#.#..#.#..#.#.#..
#..#.#..#.#..#.####.#..#..##..###..#..#."
                .to_vec()
        )
    );
}
