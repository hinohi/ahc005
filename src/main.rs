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
    view: Vec<(Vec<(usize, u32)>, Vec<(usize, u32)>)>,
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
) -> (Vec<(usize, u32)>, Vec<(usize, u32)>) {
    assert!(!map[y][x].is_block());
    let n = map.len();
    let mut horizontal = Vec::new();
    let mut d = 0;
    for x in x + 1..n {
        if let Cell::Road(n) = map[y][x] {
            d += n as u32;
            if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
                horizontal.push((i, d));
            }
        } else {
            break;
        }
    }
    let mut d = 0;
    for x in (0..x).rev() {
        if let Cell::Road(n) = map[y][x] {
            d += n as u32;
            if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
                horizontal.push((i, d));
            }
        } else {
            break;
        }
    }

    let mut vertical = Vec::new();
    let mut d = 0;
    for y in y + 1..n {
        if let Cell::Road(n) = map[y][x] {
            d += n as u32;
            if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
                vertical.push((i, d));
            }
        } else {
            break;
        }
    }
    let mut d = 0;
    for y in (0..y).rev() {
        if let Cell::Road(n) = map[y][x] {
            d += n as u32;
            if let Some((i, _)) = position.iter().find_position(|&&p| p == (x, y)) {
                vertical.push((i, d));
            }
        } else {
            break;
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

fn calc_dist_to_start(
    map: &[Vec<Cell>],
    position: &[(usize, usize)],
    start: (usize, usize),
) -> (Vec<u32>, Vec<String>) {
    use std::collections::BinaryHeap;

    let n = position.len();
    let mut dist = vec![0; n];
    let mut path = vec![String::new(); n];
    let mut heap = BinaryHeap::new();
    let mut best = vec![vec![std::u32::MAX; n]; n];
    heap.push((start, 0, String::new()));
    best[start.1][start.0] = 0;

    fn c(
        map: &[Vec<Cell>],
        best: &mut [Vec<u32>],
        heap: &mut BinaryHeap<((usize, usize), u32, String)>,
        s: &str,
        x: usize,
        y: usize,
        c: char,
        d: u32,
    ) {
        if let Cell::Road(r) = map[y][x] {
            let d = d + r as u32;
            if d < best[y][x] {
                best[y][x] = d;
                let mut s = s.to_string();
                s.push(c);
                heap.push(((x, y), d, s));
            }
        }
    }

    while let Some(((x, y), d, s)) = heap.pop() {
        if let Some((i, _)) = position.iter().find_position(|&&q| (x, y) == q) {
            dist[i] = d;
            path[i] = String::from_utf8(s.as_bytes().iter().rev().map(|c| *c).collect::<Vec<_>>())
                .unwrap();
        }
        if 0 < x {
            c(map, &mut best, &mut heap, &s, x - 1, y, 'R', d);
        }
        if x + 1 < map.len() {
            c(map, &mut best, &mut heap, &s, x + 1, y, 'L', d);
        }
        if 0 < y {
            c(map, &mut best, &mut heap, &s, x, y - 1, 'D', d);
        }
        if y + 1 < map.len() {
            c(map, &mut best, &mut heap, &s, x, y + 1, 'U', d);
        }
    }
    (dist, path)
}

fn bfs(
    intersection: &Intersection,
    watched_count: &[WatchedCount],
    start: usize,
) -> (u32, Vec<usize>) {
    use rustc_hash::FxHashMap;
    use std::collections::BinaryHeap;

    let mut best = FxHashMap::with_hasher(Default::default());
    let mut heap = BinaryHeap::new();
    best.insert(start, 0);
    heap.push((0, vec![start]));
    while let Some((d, path)) = heap.pop() {
        let p = *path.last().unwrap();
        if !watched_count[p].is_watched() {
            return (d, path);
        }
        for &(i, r) in intersection.view[p]
            .0
            .iter()
            .chain(intersection.view[p].1.iter())
        {
            let b = best.entry(i).or_insert(std::u32::MAX);
            let d = r + d;
            if d < *b {
                *b = d;
                let mut path = path.clone();
                path.push(i);
                heap.push((d, path));
            }
        }
    }
    unreachable!()
}

fn dfs(
    intersection: &Intersection,
    dist_to_start: &[u32],
    watched_count: &mut [WatchedCount],
    mut best: u32,
    dist: u32,
    path: &mut Vec<usize>,
    best_path: &mut Vec<usize>,
) -> u32 {
    let start = *path.last().unwrap();
    if watched_count.iter().all(|c| c.is_watched()) {
        let d = dist + dist_to_start[start];
        if d < best {
            *best_path = path.clone();
        }
        return d;
    }
    let (sx, sy) = intersection.position[start];
    let mut go = 0;
    for &(i, d) in intersection.view[start].0.iter() {
        if go == 1 {
            break;
        }
        if watched_count[i].is_watched() || dist + d + dist_to_start[i] >= best {
            continue;
        }
        watched_count[i].incr_h();
        watched_count[i].incr_v();
        let x0 = intersection.position[i].0.min(sx);
        let x1 = intersection.position[i].0.max(sx);
        for &(j, _) in intersection.view[start].0.iter() {
            let (jx, _) = intersection.position[j];
            if x0 < jx && jx < x1 {
                watched_count[j].incr_v();
            }
        }
        path.push(i);
        best = dfs(
            intersection,
            dist_to_start,
            watched_count,
            best,
            dist + d,
            path,
            best_path,
        )
        .min(best);
        go += 1;
        path.pop();
        for &(j, _) in intersection.view[start].0.iter() {
            let (jx, _) = intersection.position[j];
            if x0 < jx && jx < x1 {
                watched_count[j].decr_v();
            }
        }
        watched_count[i].decr_h();
        watched_count[i].decr_v();
    }
    for &(i, d) in intersection.view[start].1.iter() {
        if go == 1 {
            break;
        }
        if watched_count[i].is_watched() || dist + d + dist_to_start[i] >= best {
            continue;
        }
        watched_count[i].incr_h();
        watched_count[i].incr_v();
        let y0 = intersection.position[i].1.min(sy);
        let y1 = intersection.position[i].1.max(sy);
        for &(j, _) in intersection.view[start].1.iter() {
            let (_, jy) = intersection.position[j];
            if y0 < jy && jy < y1 {
                watched_count[j].incr_h();
            }
        }
        path.push(i);
        best = dfs(
            intersection,
            dist_to_start,
            watched_count,
            best,
            dist + d,
            path,
            best_path,
        )
        .min(best);
        go += 1;
        path.pop();
        for &(j, _) in intersection.view[start].0.iter() {
            let (_, jy) = intersection.position[j];
            if y0 < jy && jy < y1 {
                watched_count[j].decr_h();
            }
        }
        watched_count[i].decr_h();
        watched_count[i].decr_v();
    }
    if go == 0 {
        let (d, pp) = bfs(intersection, watched_count, start);
        let i = *pp.last().unwrap();
        if dist + d + dist_to_start[i] < best {
            watched_count[i].incr_h();
            watched_count[i].incr_v();
            path.extend_from_slice(&pp);
            best = dfs(
                intersection,
                dist_to_start,
                watched_count,
                best,
                dist + d,
                path,
                best_path,
            )
            .min(best);
            for _ in 0..pp.len() {
                path.pop();
            }
            watched_count[i].decr_h();
            watched_count[i].decr_v();
        }
    }
    best
}

fn main() {
    input! {
        n: usize,
        si: usize,
        sj: usize,
        map: [Bytes; n],
    }
    let start = (sj, si);
    let map = map
        .into_iter()
        .map(|row| row.into_iter().map(Cell::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (intersection, mut watched_count) = make_intersection(&map);
    let (dist_to_start, path_to_start) = calc_dist_to_start(&map, &intersection.position, start);
    let start_view = make_view(&map, &intersection.position, start.0, start.1);
    for &(i, _) in start_view.0.iter() {
        watched_count[i].incr_h();
    }
    for &(i, _) in start_view.1.iter() {
        watched_count[i].incr_v();
    }

    let mut best = std::u32::MAX;
    let mut best_path = Vec::new();
    for &(i, d) in start_view.0.iter() {
        let mut path = vec![i];
        best = dfs(
            &intersection,
            &dist_to_start,
            &mut watched_count,
            best,
            d,
            &mut path,
            &mut best_path,
        )
        .min(best);
    }
    for &(i, d) in start_view.1.iter() {
        let mut path = vec![i];
        best = dfs(
            &intersection,
            &dist_to_start,
            &mut watched_count,
            best,
            d,
            &mut path,
            &mut best_path,
        )
        .min(best);
    }

    let last = *best_path.last().unwrap();
    let (mut px, mut py) = start;
    for i in best_path {
        let (x, y) = intersection.position[i];
        if px == x {
            if y < py {
                for _ in y..py {
                    print!("U");
                }
            } else {
                for _ in py..y {
                    print!("D");
                }
            }
        } else {
            if x < px {
                for _ in x..px {
                    print!("L");
                }
            } else {
                for _ in px..x {
                    print!("R");
                }
            }
        }
        // println!("\n{} {}", x, y);
        px = x;
        py = y;
    }
    println!("{}", path_to_start[last]);
}
