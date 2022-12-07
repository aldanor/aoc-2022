use crate::utils::*;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn traverse(s: &mut &[u8], out: &mut Vec<u32>) -> u32 {
    let mut size = 0;
    while !s.is_empty() && s.get_at(0) != b'$' {
        let next = s.advance(s.memchr(b'\n') + 1);
        if s.get_at(0) != b'd' {
            size += parse_int_fast::<u32, 1, 8>(s);
        }
        *s = next;
    }
    while !s.is_empty() && s.get_at(5) != b'.' {
        *s = s.advance(s.memchr(b'\n') + 6);
        size += traverse(s, out);
    }
    if !s.is_empty() {
        *s = s.advance(8);
    }
    out.push(size);
    size
}

pub fn part1(mut s: &[u8]) -> u32 {
    s = &s[12..];
    let mut v = Vec::with_capacity(512);
    traverse(&mut s, &mut v);
    v.iter().copied().filter(|&s| s <= 100_000).sum()
}

pub fn part2(mut s: &[u8]) -> u32 {
    let mut v = Vec::with_capacity(512);
    s = &s[12..];
    let (total, required) = (70000000, 30000000);
    let size = traverse(&mut s, &mut v);
    v.iter().copied().filter(|&s| s >= size - (total - required)).min().unwrap_or(0)
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 1306611);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 13210366);
}
