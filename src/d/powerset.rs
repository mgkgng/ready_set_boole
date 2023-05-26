fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let powerset_nb = 2_i32.pow(set.len() as u32);
    let mut res = Vec::new();
    for i in 0..powerset_nb {
        let mut subset = Vec::new();
        for (j, elem) in set.iter().enumerate() {
            if i & (1 << j) != 0 {
                subset.push(*elem);
            }
        }
        res.push(subset);
    }
    res
}

// fn main() {
//     let set = vec![1, 2, 3, 7, 9];
//     let powerset = powerset(&set);
//     println!("{:?}", powerset);
// }