pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn find_first_distinct<const N: usize>(s: &[u8]) -> usize {
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
    0
}

pub fn part1(s: &[u8]) -> usize {
    find_first_distinct::<4>(s)
}

pub fn part2(s: &[u8]) -> usize {
    find_first_distinct::<14>(s)
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 1766);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 2383);
}
