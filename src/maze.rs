use disjoint_sets::UnionFind;

use crate::fenwick_tree::FenwickTree;

#[derive(Debug)]
pub struct Maze {}

impl Maze {
    pub fn build(width: usize, height: usize, _config: WallWeights) -> Option<Maze> {
        if width < 2 || height < 2 {
            return None;
        }

        let number_of_edges = ((width - 1) * height) + ((height - 1) * width);
        let number_of_cells = width * height;

        let mut _cells = UnionFind::<u32>::new(number_of_cells);
        let mut edges: Vec<bool> = vec![true; number_of_edges];
        let mut _weights = FenwickTree::<u32>::with_len(number_of_edges);

        // // If both row and column are even, element is a cell
        // // Otherwise, it's an edge
        // let grid = vec![vec![0; width * 2 - 1]; height * 2 - 1];

        // TODO: Test, remove after
        for i in 0..number_of_edges {
            println!(
                "id: {}, coord: {:?}, type: {:?}",
                i,
                get_edge_coord(width, height, i),
                get_wall_type(width, height, &edges, i)
            );
        }
        
        // TODO: Start generating maze

        Some(Maze {})
    }
}

// ========== Edge and Cell Coordinates-ID Conversion ==========

const fn get_edge_coord(width: usize, height: usize, mut id: usize) -> Option<(usize, usize)> {
    let mut row = 2 * (id / (width * 2 - 1));
    id %= width * 2 - 1;

    let col;
    if id >= (width - 1) {
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

// ========== Wall Weights ==========

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
    pub type_111x111: u32,

    //   |
    //  -  ~  -
    //   |   |
    pub type_111x011: u32,

    //   |   |
    //  -  ~
    //   |   |
    pub type_111x101: u32,

    //   |   |
    //  -  ~
    //   |
    pub type_111x100: u32,

    //   |
    //  -  ~  -
    //   |
    pub type_111x010: u32,

    //   |
    //  -  ~
    //   |
    pub type_111x000: u32,

    //   |   |
    //     ~
    //   |   |
    pub type_101x101: u32,

    //   |
    //     ~  -
    //   |   |
    pub type_101x011: u32,

    //   |
    //     ~  -
    //   |
    pub type_101x010: u32,

    //   |
    //     ~
    //   |   |
    pub type_101x001: u32,

    //   |
    //     ~
    //   |
    pub type_101x000: u32,

    //
    //  -  ~  -
    //   |   |
    pub type_011x011: u32,

    //       |
    //  -  ~  -
    //   |
    pub type_011x110: u32,

    //
    //  -  ~  -
    //   |
    pub type_011x010: u32,

    //
    //  -  ~
    //   |   |
    pub type_011x001: u32,

    //       |
    //  -  ~
    //   |
    pub type_011x100: u32,

    //
    //  -  ~
    //   |
    pub type_011x000: u32,

    //
    //  -  ~  -
    //
    pub type_010x010: u32,

    //       |
    //  -  ~
    //
    pub type_010x100: u32,

    //
    //  -  ~
    //
    pub type_010x000: u32,

    //
    //     ~
    //   |   |
    pub type_001x001: u32,

    //       |
    //     ~
    //   |
    pub type_001x100: u32,

    //
    //     ~
    //   |
    pub type_001x000: u32,
}

#[derive(Debug, Clone, Copy)]
enum WallType {
    Type111x111,
    Type111x011,
    Type111x101,
    Type111x100,
    Type111x010,
    Type111x000,
    Type101x101,
    Type101x011,
    Type101x010,
    Type101x001,
    Type101x000,
    Type011x011,
    Type011x110,
    Type011x010,
    Type011x001,
    Type011x100,
    Type011x000,
    Type010x010,
    Type010x100,
    Type010x000,
    Type001x001,
    Type001x100,
    Type001x000,
}

type NeighborsOneSided = (usize, usize, usize);
type NeighborsTwoSided = (usize, usize, usize, usize, usize, usize);

fn get_wall_type(width: usize, height: usize, edges: &Vec<bool>, id: usize) -> Option<WallType> {
    let (row, col) = get_edge_coord(width, height, id)?;

    if row == 0 || row == (height * 2 - 2) || col == 0 || col == (width * 2 - 2) {
        let neighbors;
        if row == 0 {
            let n1 = get_edge_id(width, height, row + 1, col - 1)?;
            let n2 = get_edge_id(width, height, row + 2, col)?;
            let n3 = get_edge_id(width, height, row + 1, col + 1)?;
            neighbors = (n1, n2, n3);
        } else if row == (height * 2 - 2) {
            let n1 = get_edge_id(width, height, row - 1, col - 1)?;
            let n2 = get_edge_id(width, height, row - 2, col)?;
            let n3 = get_edge_id(width, height, row - 1, col + 1)?;
            neighbors = (n1, n2, n3);
        } else if col == 0 {
            let n1 = get_edge_id(width, height, row - 1, col + 1)?;
            let n2 = get_edge_id(width, height, row, col + 2)?;
            let n3 = get_edge_id(width, height, row + 1, col + 1)?;
            neighbors = (n1, n2, n3);
        } else if col == (width * 2 - 2) {
            let n1 = get_edge_id(width, height, row - 1, col - 1)?;
            let n2 = get_edge_id(width, height, row, col - 2)?;
            let n3 = get_edge_id(width, height, row + 1, col - 1)?;
            neighbors = (n1, n2, n3);
        } else {
            return None;
        }

        if contains_wall_type_111x000(edges, neighbors) {
            return Some(WallType::Type111x000);
        } else if contains_wall_type_101x000(edges, neighbors) {
            return Some(WallType::Type101x000);
        } else if contains_wall_type_011x000(edges, neighbors) {
            return Some(WallType::Type011x000);
        } else if contains_wall_type_010x000(edges, neighbors) {
            return Some(WallType::Type010x000);
        } else if contains_wall_type_001x000(edges, neighbors) {
            return Some(WallType::Type001x000);
        } else {
            return None;
        }
    }

    let neighbors;
    if row % 2 == 0 {
        let n1 = get_edge_id(width, height, row - 1, col - 1)?;
        let n2 = get_edge_id(width, height, row - 2, col)?;
        let n3 = get_edge_id(width, height, row - 1, col + 1)?;
        let n4 = get_edge_id(width, height, row + 1, col - 1)?;
        let n5 = get_edge_id(width, height, row + 2, col)?;
        let n6 = get_edge_id(width, height, row + 1, col + 1)?;
        neighbors = (n1, n2, n3, n4, n5, n6);
    } else {
        let n1 = get_edge_id(width, height, row - 1, col - 1)?;
        let n2 = get_edge_id(width, height, row, col - 2)?;
        let n3 = get_edge_id(width, height, row + 1, col - 1)?;
        let n4 = get_edge_id(width, height, row - 1, col + 1)?;
        let n5 = get_edge_id(width, height, row, col + 2)?;
        let n6 = get_edge_id(width, height, row + 1, col + 1)?;
        neighbors = (n1, n2, n3, n4, n5, n6);
    }

    if contains_wall_type_111x111(edges, neighbors) {
        return Some(WallType::Type111x111);
    } else if contains_wall_type_111x011(edges, neighbors) {
        return Some(WallType::Type111x011);
    } else if contains_wall_type_111x101(edges, neighbors) {
        return Some(WallType::Type111x101);
    } else if contains_wall_type_111x100(edges, neighbors) {
        return Some(WallType::Type111x100);
    } else if contains_wall_type_111x010(edges, neighbors) {
        return Some(WallType::Type111x010);
    } else if contains_wall_type_101x101(edges, neighbors) {
        return Some(WallType::Type101x101);
    } else if contains_wall_type_101x011(edges, neighbors) {
        return Some(WallType::Type101x011);
    } else if contains_wall_type_101x010(edges, neighbors) {
        return Some(WallType::Type101x010);
    } else if contains_wall_type_101x001(edges, neighbors) {
        return Some(WallType::Type101x001);
    } else if contains_wall_type_011x011(edges, neighbors) {
        return Some(WallType::Type011x011);
    } else if contains_wall_type_011x110(edges, neighbors) {
        return Some(WallType::Type011x110);
    } else if contains_wall_type_011x010(edges, neighbors) {
        return Some(WallType::Type011x010);
    } else if contains_wall_type_011x001(edges, neighbors) {
        return Some(WallType::Type011x001);
    } else if contains_wall_type_011x100(edges, neighbors) {
        return Some(WallType::Type011x100);
    } else if contains_wall_type_010x010(edges, neighbors) {
        return Some(WallType::Type010x010);
    } else if contains_wall_type_010x100(edges, neighbors) {
        return Some(WallType::Type010x100);
    } else if contains_wall_type_001x001(edges, neighbors) {
        return Some(WallType::Type001x001);
    } else if contains_wall_type_001x100(edges, neighbors) {
        return Some(WallType::Type001x100);
    }

    None
}

fn contains_wall_type_111x111(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 111x111
    edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5]
}

fn contains_wall_type_111x011(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 111x011
    let v1 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 111x110
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 011x111
    let v3 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 110x111
    let v4 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_111x101(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 111x101
    let v1 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 101x111
    let v2 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_111x100(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 111x100
    let v1 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 111x001
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 100x111
    let v3 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 001x111
    let v4 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_111x010(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 111x010
    let v1 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 010x111
    let v2 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_111x000(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    // 111x000 and 000x111
    edges[neighbors.0] && edges[neighbors.1] && edges[neighbors.2]
}

fn contains_wall_type_101x101(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 101x101
    edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5]
}

fn contains_wall_type_101x011(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 101x011
    let v1 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 101x110
    let v2 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 011x101
    let v3 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 110x101
    let v4 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_101x010(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 101x010
    let v1 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 010x101
    let v2 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_101x001(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 101x001
    let v1 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 101x100
    let v2 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 001x101
    let v3 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 100x101
    let v4 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_101x000(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    // 101x000 and 000x101
    edges[neighbors.0] && !edges[neighbors.1] && edges[neighbors.2]
}

fn contains_wall_type_011x011(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 011x011
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 110x110
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_011x110(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 011x110
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 110x011
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_011x010(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 011x010
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 110x010
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 010x011
    let v3 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 010x110
    let v4 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_011x001(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 011x001
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 110x100
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 001x011
    let v3 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 100x110
    let v4 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_011x100(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 011x100
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 110x001
    let v2 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 100x011
    let v3 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 001x110
    let v4 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_011x000(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    // 011x000 and 000x011
    let v1 = !edges[neighbors.0] && edges[neighbors.1] && edges[neighbors.2];
    // 110x000 and 000x110
    let v2 = edges[neighbors.0] && !edges[neighbors.1] && !edges[neighbors.2];
    v1 || v2
}

fn contains_wall_type_010x010(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 010x010
    !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5]
}

fn contains_wall_type_010x100(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 010x100
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 010x001
    let v2 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 100x010
    let v3 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    // 001x010
    let v4 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_010x000(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    // 010x000 and 000x010
    !edges[neighbors.0] && edges[neighbors.1] && !edges[neighbors.2]
}

fn contains_wall_type_001x001(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 001x001
    let v1 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 100x100
    let v2 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_001x100(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 001x100
    let v1 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 100x001
    let v2 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_001x000(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    // 001x000 and 000x001
    let v1 = !edges[neighbors.0] && !edges[neighbors.1] && edges[neighbors.2];
    // 100x000 and 000x100
    let v2 = edges[neighbors.0] && edges[neighbors.1] && !edges[neighbors.2];
    v1 || v2
}
