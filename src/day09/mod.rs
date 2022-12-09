use crate::utils::*;

use std::iter;

use ahash::HashSetExt;
use rustc_hash::FxHashSet;

type X = i16;

pub fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn parse_inputs<T: Copy + Default + 'static>(
    mut s: &[u8], dirs: [T; 4],
) -> impl Iterator<Item = (T, usize)> + '_ {
    let mut dp = [T::default(); 256];
    dp[b'U' as usize] = dirs[0];
    dp[b'R' as usize] = dirs[1];
    dp[b'D' as usize] = dirs[2];
    dp[b'L' as usize] = dirs[3];
    iter::from_fn(move || {
        (!s.is_empty()).then(|| {
            let dp = dp[usize::from(s.get_at(0))];
            let mut x = usize::from(s.get_at(2) - b'0');
            if s.get_at(3) == b'\n' {
                s = s.advance(4);
            } else {
                x = x * 10 + usize::from(s.get_at(3) - b'0');
                s = s.advance(5);
            }
            (dp, x)
        })
    })
}

// 812
// 703
// 654
const DIRS: [(X, X); 9] =
    [(0, 0), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

fn build_map() -> [[(usize, usize, (X, X)); 9]; 9] {
    let mut out = [[(0, 0, (0, 0)); 9]; 9];
    for dh_id in 0..9 {
        let dh: (X, X) = DIRS[dh_id];
        for ht_id in 0..9 {
            let h0: (X, X) = DIRS[ht_id];
            let h1 = (h0.0 + dh.0, h0.1 + dh.1);
            let dt = if h1.0.abs() <= 1 && h1.1.abs() <= 1 {
                (0, 0)
            } else {
                (h1.0.signum(), h1.1.signum())
            };
            let (mut ht_id_new, mut dt_id) = (0, 0);
            let ht_new = (h1.0 - dt.0, h1.1 - dt.1);
            for s in 0..9 {
                if ht_new == DIRS[s] {
                    ht_id_new = s;
                    break;
                }
            }
            for s in 0..9 {
                if dt == DIRS[s] {
                    dt_id = s;
                    break;
                }
            }
            out[dh_id][ht_id] = (ht_id_new, dt_id, dt);
        }
    }
    out
}

pub fn part1(s: &[u8]) -> usize {
    let map = build_map();
    let mut ht_id = 0_usize;
    let mut tail: (X, X) = (0, 0);
    let mut set = FxHashSet::with_capacity(1 << 16);
    set.insert(tail);
    for (dh_id, n) in parse_inputs(s, [1_usize, 3, 5, 7]) {
        for _ in 0..n {
            let (ht_id_new, dir, dt) = map[dh_id][ht_id];
            ht_id = ht_id_new;
            if dir == 0 {
                continue;
            }
            tail = (tail.0 + dt.0, tail.1 + dt.1);
            set.insert(tail);
        }
    }
    set.len()
}

pub fn part2(s: &[u8]) -> usize {
    let map = build_map();
    let mut ht_ids = [0_usize; 9];
    let mut tail: (X, X) = (0, 0);
    let mut set = FxHashSet::with_capacity(1 << 16);
    set.insert(tail);
    for (dh_id, n) in parse_inputs(s, [1_usize, 3, 5, 7]) {
        'outer: for _ in 0..n {
            let mut dir = dh_id;
            let mut dt = (0, 0);
            for i in 0..9 {
                (ht_ids[i], dir, dt) = map[dir][ht_ids[i]];
                if dir == 0 {
                    continue 'outer;
                }
            }
            tail = (tail.0 + dt.0, tail.1 + dt.1);
            set.insert(tail);
        }
    }
    set.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(input()), 5735);
}

#[test]
fn test_part2() {
    assert_eq!(part2(input()), 2478);
}
