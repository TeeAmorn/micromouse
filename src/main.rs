// use maze::fenwick_tree::FenwickTree;
use maze::maze::{Maze, WallWeights};

fn main() {
    // let mut tree = FenwickTree::<i32>::with_len(10);
    // tree.add(0, 2).unwrap();
    // tree.add(1, 2).unwrap();
    // tree.add(2, 2).unwrap();
    // tree.add(3, 2).unwrap();
    // tree.add(4, 2).unwrap();
    // tree.add(5, 2).unwrap();
    // tree.add(6, 2).unwrap();
    // tree.add(6, 1).unwrap();
    // tree.subtract(6, 2).unwrap();
    // tree.add(7, 2).unwrap();
    // tree.add(8, 2).unwrap();
    // tree.add(9, 2).unwrap();
    // tree.add(9, 1).unwrap();
    // tree.subtract(9, 2).unwrap();
    // println!("{:?}", tree.get_sums().unwrap());

    // let sum = 3;
    // let u = tree.get_lower(sum).unwrap();
    // println!("sum: {}, index: {}", sum, u);

    let config = WallWeights {
        type_111_111: 1,
        type_111_101: 1,
        type_111_100: 1,
        type_111_010: 1,
        type_111_000: 1,
        type_101_101: 1,
        type_101_011: 1,
        type_101_010: 1,
        type_101_001: 1,
        type_101_000: 1,
        type_011_011: 1,
        type_011_110: 1,
        type_011_010: 1,
        type_011_001: 1,
        type_011_100: 1,
        type_011_000: 1,
        type_010_010: 1,
        type_010_100: 1,
        type_010_000: 1,
        type_001_001: 1,
        type_001_100: 1,
        type_001_000: 1,
    };
    println!("{:?}", Maze::build(4, 5, config));
}
