use arrayvec::ArrayVec;

use crate::utils::*;

const K: usize = 16;
const N: usize = 64;
type Stack = ArrayVec<u8, N>;
type Stacks = ArrayVec<Stack, K>;
type Tops = ArrayVec<ArrayVec<(usize, usize), K>, K>;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_stacks(s: &mut &[u8]) -> Stacks {
    let line_len = 1 + s.memchr(b'\n');
    let n_stacks = line_len >> 2;
    let stacks_len = s.memchr(b'1') - 1;
    let max_height = stacks_len / line_len;
    debug_assert_eq!(line_len % 4, 0);
    debug_assert_eq!(stacks_len % line_len, 0);
    debug_assert!(n_stacks <= 9);

    let mut stacks = ArrayVec::new();
    for _ in 0..n_stacks {
        stacks.push(ArrayVec::new());
    }
    let mut row_end = stacks_len;
    for _ in 0..max_height {
        let row = &s[row_end - line_len..row_end];
        for j in 0..n_stacks {
            let c = row.get_at((j << 2) + 1);
            if c != b' ' {
                stacks[j].push(c);
            }
        }
        row_end -= line_len;
    }

    *s = s.advance(stacks_len + line_len + 1);
    stacks
}

fn parse_moves(mut s: &[u8]) -> Vec<(usize, usize, usize)> {
    let mut moves = Vec::with_capacity(1024);
    while s.len() > 1 {
        s = s.advance(5);
        let mut n = s.get_at(0) - b'0';
        if (b'0'..=b'9').contains(&s.get_at(1)) {
            n = n * 10 + s.get_at(1) - b'0';
            s = s.advance(1);
        }
        let n = usize::from(n);
        let from = usize::from(s.get_at(7) - b'0' - 1);
        let to = usize::from(s.get_at(12) - b'0' - 1);
        s = s.advance(14);
        moves.push((n, from, to));
    }
    moves
}

fn tops_to_answer(tops: &Tops, stacks: &Stacks) -> String {
    let mut out = ArrayVec::<char, K>::new();
    for _ in 0..stacks.len() {
        out.push(' ');
    }
    for (i, top) in tops.iter().enumerate() {
        for &(pos, id) in top {
            out[id as usize] = stacks[i][stacks[i].len() - 1 - pos] as char;
        }
    }
    let out = String::from_iter(out.into_iter());
    out
}

fn solve(mut s: &[u8], pos_fn: impl Fn(usize, usize) -> usize) -> String {
    let stacks = parse_stacks(&mut s);
    let moves = parse_moves(s);
    let mut tops = Tops::new();
    for i in 0..stacks.len() {
        tops.push(ArrayVec::new());
        tops[i].push((0, i));
    }
    for &(n, from, to) in moves.iter().rev() {
        let n_from = tops[from].len();
        for i in 0..n_from {
            tops[from][i].0 += n;
        }
        let n_to = tops[to].len();
        for i in (0..n_to).rev() {
            let (pos, id) = tops[to][i];
            if pos < n {
                tops[to].swap_remove(i);
                tops[from].push((pos_fn(pos, n), id));
            } else {
                tops[to][i].0 -= n;
            }
        }
    }
    tops_to_answer(&tops, &stacks)
}

pub fn part1(s: &[u8]) -> String {
    solve(s, |pos, n| n - pos - 1)
}

pub fn part2(s: &[u8]) -> String {
    solve(s, |pos, _n| pos)
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), "SHMSDGZVC");
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), "VRZGHDFBQ");
}
