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

fn main() {
    input! {
        n: usize,
        _start: (usize, usize),
        map: [Bytes; n],
    }
    let map = map
        .into_iter()
        .map(|row| row.into_iter().map(Cell::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{:?}", map);
}
