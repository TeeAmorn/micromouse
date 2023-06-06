use disjoint_sets::UnionFind;
use rand::Rng;

use crate::fenwick_tree::FenwickTree;

#[derive(Debug)]
pub struct Maze {}

impl Maze {
    pub fn build(width: usize, height: usize, config: WallWeights) -> Option<Maze> {
        if width < 2 || height < 2 {
            return None;
        }

        let number_of_edges = ((width - 1) * height) + ((height - 1) * width);
        let number_of_cells = width * height;

        let mut cells = UnionFind::<usize>::new(number_of_cells);
        let mut edges: Vec<bool> = vec![true; number_of_edges];
        let mut weights = FenwickTree::<u32>::with_len(number_of_edges);

        // Initialize weight of every edge
        for i in 0..number_of_edges {
            let _ = weights.set(i, get_weight(width, height, &edges, &config, i)?);
        }

        // Start generating maze
        for _ in 0..number_of_edges {
            // Select and set weight of random edge to 0
            let rand_num = rand::thread_rng().gen_range(1..=weights.get_final_sum());
            let edge_id_to_remove = weights.get_lower(rand_num).ok()?;
            let _ = weights.set(edge_id_to_remove, 0);

            // Get coordinates of current edge
            let (row, col) = get_edge_coord(width, height, edge_id_to_remove)?;

            // Determine if edge should be removed by looking at adjacent cells
            let cell_a;
            let cell_b;
            if row % 2 == 0 {
                cell_a = get_cell_id(width, height, row, col - 1)?;
                cell_b = get_cell_id(width, height, row, col + 1)?;
            } else {
                cell_a = get_cell_id(width, height, row - 1, col)?;
                cell_b = get_cell_id(width, height, row + 1, col)?;
            }
            if !cells.union(cell_a, cell_b) {
                continue;
            }
            edges[edge_id_to_remove] = false;

            // Find neighboring edges
            let neighbors;
            if row % 2 == 0 {
                if row == 0 {
                    let n1 = get_edge_id(width, height, row + 1, col - 1)?;
                    let n2 = get_edge_id(width, height, row + 2, col)?;
                    let n3 = get_edge_id(width, height, row + 1, col + 1)?;
                    neighbors = vec![n1, n2, n3];
                } else if row == (2 * height - 2) {
                    let n1 = get_edge_id(width, height, row - 1, col - 1)?;
                    let n2 = get_edge_id(width, height, row - 2, col)?;
                    let n3 = get_edge_id(width, height, row - 1, col + 1)?;
                    neighbors = vec![n1, n2, n3];
                } else {
                    let n1 = get_edge_id(width, height, row - 1, col - 1)?;
                    let n2 = get_edge_id(width, height, row - 2, col)?;
                    let n3 = get_edge_id(width, height, row - 1, col + 1)?;
                    let n4 = get_edge_id(width, height, row + 1, col - 1)?;
                    let n5 = get_edge_id(width, height, row + 2, col)?;
                    let n6 = get_edge_id(width, height, row + 1, col + 1)?;
                    neighbors = vec![n1, n2, n3, n4, n5, n6];
                }
            } else {
                if col == 0 {
                    let n1 = get_edge_id(width, height, row - 1, col + 1)?;
                    let n2 = get_edge_id(width, height, row, col + 2)?;
                    let n3 = get_edge_id(width, height, row + 1, col + 1)?;
                    neighbors = vec![n1, n2, n3];
                } else if col == (2 * width - 2) {
                    let n1 = get_edge_id(width, height, row - 1, col - 1)?;
                    let n2 = get_edge_id(width, height, row, col - 2)?;
                    let n3 = get_edge_id(width, height, row + 1, col - 1)?;
                    neighbors = vec![n1, n2, n3];
                } else {
                    let n1 = get_edge_id(width, height, row - 1, col - 1)?;
                    let n2 = get_edge_id(width, height, row, col - 2)?;
                    let n3 = get_edge_id(width, height, row + 1, col - 1)?;
                    let n4 = get_edge_id(width, height, row - 1, col + 1)?;
                    let n5 = get_edge_id(width, height, row + 1, col + 1)?;
                    let n6 = get_edge_id(width, height, row, col + 2)?;
                    neighbors = vec![n1, n2, n3, n4, n5, n6];
                }
            }

            // Update weight of each neighbor
            for id in neighbors {
                let _ = weights.set(id, get_weight(width, height, &edges, &config, id)?);
            }
        }

        Some(Maze {})
    }
}

// ========== Edge and Cell Coordinates-ID Conversion ==========

fn get_edge_coord(width: usize, height: usize, mut id: usize) -> Option<(usize, usize)> {
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
        println!("Given row {} or col {} too large", row, col);
        return None;
    }

    Some((row, col))
}

fn get_cell_coord(width: usize, height: usize, mut id: usize) -> Option<(usize, usize)> {
    let row = 2 * (id / width);
    id %= width;
    let col = id * 2;

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        println!("Given row {} or col {} too large", row, col);
        return None;
    }

    Some((row, col))
}

fn get_edge_id(width: usize, height: usize, row: usize, col: usize) -> Option<usize> {
    if !((row % 2 == 0) ^ (col % 2 == 0)) {
        println!("Given row {} or col {} is not edge", row, col);
        return None;
    }

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        println!("Given row {} or col {} too large", row, col);
        return None;
    }

    let vertical_edges = ((row + 1) / 2 * (width - 1)) + (((row + 1) % 2) * col / 2);
    let horizontal_edges = (row / 2 * width) + ((row % 2) * col / 2);
    Some(vertical_edges + horizontal_edges)
}

fn get_cell_id(width: usize, height: usize, row: usize, col: usize) -> Option<usize> {
    if (row % 2 != 0) || (col % 2 != 0) {
        println!("Given row {} or col {} is not edge", row, col);
        return None;
    }

    if (row >= height * 2 - 1) || (col >= width * 2 - 1) {
        println!("Given row {} or col {} too large", row, col);
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

    //
    //     ~
    //
    pub type_000x000: u32,
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
    Type000x000,
}

type NeighborsOneSided = (usize, usize, usize);
type NeighborsTwoSided = (usize, usize, usize, usize, usize, usize);

fn get_weight(
    width: usize,
    height: usize,
    edges: &Vec<bool>,
    config: &WallWeights,
    id: usize,
) -> Option<u32> {
    match get_wall_type(width, height, edges, id)? {
        WallType::Type111x111 => Some(config.type_111x111),
        WallType::Type111x011 => Some(config.type_111x011),
        WallType::Type111x101 => Some(config.type_111x101),
        WallType::Type111x100 => Some(config.type_111x100),
        WallType::Type111x010 => Some(config.type_111x010),
        WallType::Type111x000 => Some(config.type_111x000),
        WallType::Type101x101 => Some(config.type_101x101),
        WallType::Type101x011 => Some(config.type_101x011),
        WallType::Type101x010 => Some(config.type_101x010),
        WallType::Type101x001 => Some(config.type_101x001),
        WallType::Type101x000 => Some(config.type_101x000),
        WallType::Type011x011 => Some(config.type_011x011),
        WallType::Type011x110 => Some(config.type_011x110),
        WallType::Type011x010 => Some(config.type_011x010),
        WallType::Type011x001 => Some(config.type_011x001),
        WallType::Type011x100 => Some(config.type_011x100),
        WallType::Type011x000 => Some(config.type_011x000),
        WallType::Type010x010 => Some(config.type_010x010),
        WallType::Type010x100 => Some(config.type_010x100),
        WallType::Type010x000 => Some(config.type_010x000),
        WallType::Type001x001 => Some(config.type_001x001),
        WallType::Type001x100 => Some(config.type_001x100),
        WallType::Type001x000 => Some(config.type_001x000),
        WallType::Type000x000 => Some(config.type_000x000),
    }
}

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
            println!("SHOULD NEVER REACH HERE!");
            return None;
        }

        if contains_wall_type_111(edges, neighbors) {
            return Some(WallType::Type111x000);
        } else if contains_wall_type_101(edges, neighbors) {
            return Some(WallType::Type101x000);
        } else if contains_wall_type_011(edges, neighbors) {
            return Some(WallType::Type011x000);
        } else if contains_wall_type_010(edges, neighbors) {
            return Some(WallType::Type010x000);
        } else if contains_wall_type_001(edges, neighbors) {
            return Some(WallType::Type001x000);
        } else if contains_wall_type_000(edges, neighbors) {
            return Some(WallType::Type000x000);
        } else {
            println!("Cannot find matching EDGE wall type");
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
    } else if contains_wall_type_111x000(edges, neighbors) {
        return Some(WallType::Type111x000);
    } else if contains_wall_type_101x000(edges, neighbors) {
        return Some(WallType::Type101x000);
    } else if contains_wall_type_011x000(edges, neighbors) {
        return Some(WallType::Type011x000);
    } else if contains_wall_type_010x000(edges, neighbors) {
        return Some(WallType::Type010x000);
    } else if contains_wall_type_001x000(edges, neighbors) {
        return Some(WallType::Type001x000);
    } else if contains_wall_type_000x000(edges, neighbors) {
        return Some(WallType::Type000x000);
    }

    println!("Cannot find matching MIXED wall type");
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

fn contains_wall_type_111(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
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

fn contains_wall_type_101(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
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

fn contains_wall_type_011(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
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

fn contains_wall_type_010(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
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

fn contains_wall_type_001(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    // 001x000 and 000x001
    let v1 = !edges[neighbors.0] && !edges[neighbors.1] && edges[neighbors.2];
    // 100x000 and 000x100
    let v2 = edges[neighbors.0] && edges[neighbors.1] && !edges[neighbors.2];
    v1 || v2
}

fn contains_wall_type_000(edges: &Vec<bool>, neighbors: NeighborsOneSided) -> bool {
    !edges[neighbors.0] && !edges[neighbors.1] && !edges[neighbors.2]
}

fn contains_wall_type_111x000(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 111x010
    let v1 = edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 010x111
    let v2 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_101x000(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 101x000
    let v1 = edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 000x101
    let v2 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_011x000(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 011x000
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 000x011
    let v2 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && edges[neighbors.5];
    // 110x000
    let v3 = edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 000x110
    let v4 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_010x000(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 010x000
    let v1 = !edges[neighbors.0]
        && edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 000x010
    let v2 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2
}

fn contains_wall_type_001x000(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    // 001x000
    let v1 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 000x001
    let v2 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && edges[neighbors.5];
    // 100x000
    let v3 = edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    // 000x100
    let v4 = !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5];
    v1 || v2 || v3 || v4
}

fn contains_wall_type_000x000(edges: &Vec<bool>, neighbors: NeighborsTwoSided) -> bool {
    !edges[neighbors.0]
        && !edges[neighbors.1]
        && !edges[neighbors.2]
        && !edges[neighbors.3]
        && !edges[neighbors.4]
        && !edges[neighbors.5]
}
