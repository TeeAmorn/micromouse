use disjoint_sets::UnionFind;

use crate::fenwick_tree::FenwickTree;

#[derive(Debug)]
pub struct Maze {}

impl Maze {
    pub fn build(width: usize, height: usize, _config: WallWeights) -> Option<Maze> {
        if width < 1 || height < 1 {
            return None;
        }

        let number_of_edges = ((width - 1) * height) + ((height - 1) * width);
        let number_of_cells = width * height;

        let mut _cells = UnionFind::<u32>::new(number_of_cells);
        let mut _edges: Vec<bool> = vec![true; number_of_edges];
        let mut _weights = FenwickTree::<u32>::with_len(number_of_edges);

        // // If both row and column are even, element is a cell
        // // Otherwise, it's an edge
        // let grid = vec![vec![0; width * 2 - 1]; height * 2 - 1];

        // TODO: Implement wall matching logic

        Some(Maze {})
    }
}

const fn get_edge_coord(width: usize, height: usize, mut id: usize) -> Option<(usize, usize)> {
    let mut row = 2 * (id / (width * 2 - 1));
    id %= width * 2 - 1;

    let col;
    if id > (width - 1) {
        row += 1;
        id -= width - 1;
        col = id * 2;
    } else {
        col = id * 2 + 1;
    }

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        return None;
    }

    Some((row, col))
}

const fn get_cell_coord(width: usize, height: usize, mut id: usize) -> Option<(usize, usize)> {
    let row = 2 * (id / width);
    id %= width;
    let col = id * 2;

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        return None;
    }

    Some((row, col))
}

const fn get_edge_id(width: usize, height: usize, row: usize, col: usize) -> Option<usize> {
    if !((row % 2 == 0) ^ (col % 2 == 0)) {
        return None;
    }

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        return None;
    }

    let vertical_edges = ((row + 1) / 2 * (width - 1)) + (((row + 1) % 2) * col / 2);
    let horizontal_edges = (row / 2 * width) + ((row % 2) * col / 2);
    Some(vertical_edges + horizontal_edges)
}

const fn get_cell_id(width: usize, height: usize, row: usize, col: usize) -> Option<usize> {
    if (row % 2 != 0) || (col % 2 != 0) {
        return None;
    }

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        return None;
    }

    Some(row / 2 * width + col / 2)
}

pub struct WallWeights {
    // Name Format: type[0][1][2][3]_[4][5][6]
    //     0   3
    //     |   |
    // 1 -   ~   - 4
    //     |   |
    //     2   5

    //   |   |
    //  -  ~  -
    //   |   |
    pub type_111_111: u32,

    //   |   |
    //  -  ~
    //   |   |
    pub type_111_101: u32,

    //   |   |
    //  -  ~
    //   |
    pub type_111_100: u32,

    //   |
    //  -  ~  -
    //   |
    pub type_111_010: u32,

    //   |
    //  -  ~
    //   |
    pub type_111_000: u32,

    //   |   |
    //     ~
    //   |   |
    pub type_101_101: u32,

    //   |
    //     ~  -
    //   |   |
    pub type_101_011: u32,

    //   |
    //     ~  -
    //   |
    pub type_101_010: u32,

    //   |
    //     ~
    //   |   |
    pub type_101_001: u32,

    //   |
    //     ~
    //   |
    pub type_101_000: u32,

    //
    //  -  ~  -
    //   |   |
    pub type_011_011: u32,

    //       |
    //  -  ~  -
    //   |
    pub type_011_110: u32,

    //
    //  -  ~  -
    //   |
    pub type_011_010: u32,

    //
    //  -  ~
    //   |   |
    pub type_011_001: u32,

    //       |
    //  -  ~
    //   |
    pub type_011_100: u32,

    //
    //  -  ~
    //   |
    pub type_011_000: u32,

    //
    //  -  ~  -
    //
    pub type_010_010: u32,

    //       |
    //  -  ~
    //
    pub type_010_100: u32,

    //
    //  -  ~
    //
    pub type_010_000: u32,

    //
    //     ~
    //   |   |
    pub type_001_001: u32,

    //       |
    //     ~
    //   |
    pub type_001_100: u32,

    //
    //     ~
    //   |
    pub type_001_000: u32,
}

