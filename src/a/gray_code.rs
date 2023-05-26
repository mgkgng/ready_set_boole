fn gray_code(n: u32) -> u32 {
    return n ^ (n >> 1);
}

// fn main() {
//     println!("{}", gray_code(0)); // 0
//     println!("{}", gray_code(1)); // 1
//     println!("{}", gray_code(2)); // 3
//     println!("{}", gray_code(3)); // 2
//     println!("{}", gray_code(4)); // 6
//     println!("{}", gray_code(5)); // 7
//     println!("{}", gray_code(6)); // 5
//     println!("{}", gray_code(7)); // 4
//     println!("{}", gray_code(8)); // 12
// }