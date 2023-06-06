use maze::fenwick_tree::FenwickTree;
use maze::maze::{Maze, WallWeights};

fn main() {
    // let mut tree = FenwickTree::<i32>::with_len(7);
    // tree.set(0, 2).unwrap();
    // tree.set(1, 2).unwrap();
    // tree.set(2, 2).unwrap();
    // tree.set(3, 2).unwrap();
    // tree.set(4, 0).unwrap();
    // tree.set(5, 2).unwrap();
    // tree.set(6, 0).unwrap();
    // println!("{:?}", tree.get_sums().unwrap());

    // let sum = 10;
    // let u = tree.get_lower(sum).unwrap();
    // println!("sum: {}, index: {}", sum, u);

    let config = WallWeights {
        type_111x111: 1,
        type_111x011: 2,
        type_111x101: 3,
        type_111x100: 4,
        type_111x010: 5,
        type_111x000: 6,
        type_101x101: 7,
        type_101x011: 8,
        type_101x010: 9,
        type_101x001: 10,
        type_101x000: 11,
        type_011x011: 12,
        type_011x110: 13,
        type_011x010: 14,
        type_011x001: 15,
        type_011x100: 16,
        type_011x000: 17,
        type_010x010: 18,
        type_010x100: 19,
        type_010x000: 20,
        type_001x001: 21,
        type_001x100: 22,
        type_001x000: 23,
        type_000x000: 24,
    };
    println!("{:?}", Maze::build(10, 10, config));
}
