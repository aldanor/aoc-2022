use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_range(s: &mut &[u8]) -> [i16; 2] {
    let x = [s.get_at(0), s.get_at(1)];
    if x[1] & 0b10000 != 0 {
        let y = [s.get_at(3), s.get_at(4)];
        *s = s.advance(6);
        [i16::from_be_bytes(x), i16::from_be_bytes(y)]
    } else {
        let y = [s.get_at(2), s.get_at(3)];
        if y[1] & 0b10000 != 0 {
            *s = s.advance(5);
            [x[0] as _, i16::from_be_bytes(y)]
        } else {
            *s = s.advance(4);
            [x[0] as _, y[0] as _]
        }
    }
}

pub fn part1(mut s: &[u8]) -> u16 {
    let mut total = 0;
    while s.len() > 1 {
        let x = parse_range(&mut s);
        let y = parse_range(&mut s);
        let d = [i32::from(x[0] - y[0]), i32::from(x[1] - y[1])];
        total += u16::from(d[0] * d[1] <= 0);
    }
    total
}

pub fn part2(mut s: &[u8]) -> u16 {
    let mut total = 0;
    while s.len() > 1 {
        let x = parse_range(&mut s);
        let y = parse_range(&mut s);
        total += u16::from(x[0] <= y[1] && y[0] <= x[1]);
    }
    total
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 530);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 903);
}
