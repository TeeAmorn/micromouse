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
        type_111x111: 1,
        type_111x011: 1,
        type_111x101: 1,
        type_111x100: 1,
        type_111x010: 1,
        type_111x000: 1,
        type_101x101: 1,
        type_101x011: 1,
        type_101x010: 1,
        type_101x001: 1,
        type_101x000: 1,
        type_011x011: 1,
        type_011x110: 1,
        type_011x010: 1,
        type_011x001: 1,
        type_011x100: 1,
        type_011x000: 1,
        type_010x010: 1,
        type_010x100: 1,
        type_010x000: 1,
        type_001x001: 1,
        type_001x100: 1,
        type_001x000: 1,
    };
    println!("{:?}", Maze::build(4, 5, config));
}
