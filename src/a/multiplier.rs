mod adder;
use adder::adder;

fn multiplier(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0;
    for n in 0..32 {
        let mu: u32 = if ((b >> n) & 1) != 0 { !0u32 } else { 0 };
        let m = a & mu;
        res = adder(res, m << n);
    }
    res
}

// fn main() {
//     let a = 24123;
//     let b = 24;
//     let res = multiplier(a, b);
//     println!("Compare: {} / {}", res, a * b);
// }