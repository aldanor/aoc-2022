use core_simd::simd::{u8x32, SimdPartialEq};

type S = u8x32;
const W: usize = 32;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn find_first_distinct_naive<const N: usize>(s: &[u8]) -> usize {
    // naive but simple
    let mut eq = [0u8; N];
    for i in 0..N - 1 {
        for j in i + 1..N {
            eq[i] |= (s[i] == s[j]) as u8;
        }
    }
    for i in N..s.len() {
        if eq.iter().all(|&x| x == 0) {
            return i;
        }
        eq.rotate_left(1);
        for j in 0..N - 1 {
            eq[j] |= (s[i] == s[i - N + 1 + j]) as u8;
        }
        eq[N - 1] = 0;
    }
    usize::MAX
}

fn find_first_distinct_lookup<const N: usize>(s: &[u8]) -> usize {
    // faster for larger N
    let mut prev = [0; 256];
    let mut start = N - 1;
    for (i, &c) in s.iter().enumerate() {
        let p = &mut prev[c as usize];
        if i - *p < N {
            // overlap
            start = start.max(*p + 1);
        } else {
            if i >= start + N - 1 {
                // answer
                return start + N;
            }
        }
        *p = i;
    }
    usize::MAX
}

pub fn part1(mut s: &[u8]) -> usize {
    let mut offset = 0;
    loop {
        let [a, b, c, d] = std::array::from_fn(|i| S::from_slice(&s[i..W + i]));
        let eq =
            a.simd_eq(b) | a.simd_eq(c) | b.simd_eq(c) | a.simd_eq(d) | b.simd_eq(d) | c.simd_eq(d);
        if !eq.all() {
            break;
        }
        s = &s[W..];
        offset += W;
    }
    find_first_distinct_naive::<4>(s) + offset // naive is faster than lookup here (?)
}

pub fn part2(mut s: &[u8]) -> usize {
    let mut offset = 0;
    loop {
        let [a, b, c, d] = std::array::from_fn(|i| S::from_slice(&s[i..W + i]));
        let eq =
            a.simd_eq(b) | a.simd_eq(c) | b.simd_eq(c) | a.simd_eq(d) | b.simd_eq(d) | c.simd_eq(d);
        if !eq.all() {
            break;
        }
        s = &s[W..];
        offset += W;
    }
    find_first_distinct_lookup::<14>(s) + offset // lots of work here, so we'll use the lookup
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 1766);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 2383);
}
