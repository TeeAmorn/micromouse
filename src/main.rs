use maze::fenwick_tree::FenwickTree;

fn main() {
    let mut tree = FenwickTree::<i32>::with_len(10);
    tree.add(0, 2).unwrap();
    tree.add(1, 2).unwrap();
    tree.add(2, 2).unwrap();
    tree.add(3, 2).unwrap();
    tree.add(4, 2).unwrap();
    tree.add(5, 2).unwrap();
    tree.add(6, 2).unwrap();
    tree.add(7, 2).unwrap();
    tree.add(8, 2).unwrap();
    tree.add(9, 2).unwrap();
    println!("{:?}", tree.get_sums().unwrap());

    let sum = 3;
    let u = tree.get_lower(sum).unwrap();
    println!("sum: {}, index: {}", sum, u);
}
