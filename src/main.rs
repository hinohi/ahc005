use itertools::Itertools;
use proconio::{input, marker::Bytes};

#[derive(Debug, Copy, Clone)]
enum Cell {
    Block,
    Road(u8),
}

impl From<u8> for Cell {
    fn from(b: u8) -> Self {
        match b {
            b'#' => Cell::Block,
            n => Cell::Road(n - b'0'),
        }
    }
}

impl Cell {
    #[inline]
    fn is_block(self) -> bool {
        matches!(self, Cell::Block)
    }
}

#[derive(Debug)]
struct Intersection {
    position: Vec<(usize, usize)>,
    /// (horizontal, vertical)
    view: Vec<(Vec<usize>, Vec<usize>)>,
}

#[derive(Debug, Copy, Clone)]
enum WatchedCount {
    /// 角
    Corner(u8),
    /// 十字路
    J { h: u8, v: u8 },
    /// T字路
    T1 { h: u8 },
    /// ト字路
    T2 { v: u8 },
}

impl WatchedCount {
    fn is_watched(self) -> bool {
        use WatchedCount::*;
        match self {
            Corner(n) => n > 0,
            J { h, v } => h > 0 && v > 0,
            T1 { h } => h > 0,
            T2 { v } => v > 0,
        }
    }

    fn incr_h(&mut self) {
        use WatchedCount::*;
        match self {
            Corner(n) => *n += 1,
            J { h, .. } => *h += 1,
            T1 { h } => *h += 1,
            T2 { .. } => (),
        }
    }

    fn incr_v(&mut self) {
        use WatchedCount::*;
        match self {
            Corner(n) => *n += 1,
            J { v, .. } => *v += 1,
            T1 { .. } => (),
            T2 { v } => *v += 1,
        }
    }

    fn decr_h(&mut self) {
        use WatchedCount::*;
        match self {
            Corner(n) => *n -= 1,
            J { h, .. } => *h -= 1,
            T1 { h } => *h -= 1,
            T2 { .. } => (),
        }
    }

    fn decr_v(&mut self) {
        use WatchedCount::*;
        match self {
            Corner(n) => *n -= 1,
            J { v, .. } => *v -= 1,
            T1 { .. } => (),
            T2 { v } => *v -= 1,
        }
    }
}

fn cell_flag(map: &[Vec<Cell>], x: usize, y: usize) -> u8 {
    let mut flag = 0;
    if 0 < x && !map[y][x - 1].is_block() {
        flag |= 1;
    }
    if x + 1 < map.len() && !map[y][x + 1].is_block() {
        flag |= 2;
    }
    if 0 < y && !map[y - 1][x].is_block() {
        flag |= 4;
    }
    if y + 1 < map.len() && !map[y + 1][x].is_block() {
        flag |= 8;
    }
    flag
}

fn make_view(
    map: &[Vec<Cell>],
    position: &[(usize, usize)],
    x: usize,
    y: usize,
) -> (Vec<usize>, Vec<usize>) {
    let n = map.len();
    let mut horizontal = Vec::new();
    for x in x + 1..n {
        if map[y][x].is_block() {
            break;
        }
        if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
            horizontal.push(i);
        }
    }
    for x in (0..x).rev() {
        if map[y][x].is_block() {
            break;
        }
        if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
            horizontal.push(i);
        }
    }

    let mut vertical = Vec::new();
    for y in y + 1..n {
        if map[y][x].is_block() {
            break;
        }
        if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
            vertical.push(i);
        }
    }
    for y in (0..y).rev() {
        if map[y][x].is_block() {
            break;
        }
        if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
            vertical.push(i);
        }
    }

    (horizontal, vertical)
}

fn make_intersection(map: &[Vec<Cell>]) -> (Intersection, Vec<WatchedCount>) {
    let n = map.len();
    let mut position = Vec::new();
    let mut watched_count = Vec::new();
    for y in 0..n {
        for x in 0..n {
            if map[y][x].is_block() {
                continue;
            }
            match cell_flag(map, x, y) {
                5 | 9 | 6 | 10 => {
                    position.push((x, y));
                    watched_count.push(WatchedCount::Corner(0));
                }
                7 | 11 => {
                    position.push((x, y));
                    watched_count.push(WatchedCount::T1 { h: 0 });
                }
                13 | 14 => {
                    position.push((x, y));
                    watched_count.push(WatchedCount::T2 { v: 0 });
                }
                15 => {
                    position.push((x, y));
                    watched_count.push(WatchedCount::J { h: 0, v: 0 });
                }
                0 | 1 | 2 | 4 | 8 | 3 | 12 => (),
                _ => unreachable!(),
            }
        }
    }

    let mut view = Vec::with_capacity(position.len());
    for &(x, y) in position.iter() {
        view.push(make_view(map, &position, x, y));
    }
    (Intersection { position, view }, watched_count)
}

fn main() {
    input! {
        n: usize,
        start: (usize, usize),
        map: [Bytes; n],
    }
    let map = map
        .into_iter()
        .map(|row| row.into_iter().map(Cell::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (intersection, mut watched_count) = make_intersection(&map);
    let start_view = make_view(&map, &intersection.position, start.0, start.1);
    for &i in start_view.0.iter() {
        watched_count[i].incr_h();
    }
    for &i in start_view.1.iter() {
        watched_count[i].incr_v();
    }
    println!("{:?}", intersection.position.len());
    println!("{:?}", watched_count);
}
