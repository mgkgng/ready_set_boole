fn half_adder(a: u32, b: u32) -> (u32, u32) {
    return (a ^ b, a & b);
}

fn full_adder(a: u32, b: u32, c: u32) -> (u32, u32) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    return (sum2, carry1 | carry2);
}

fn adder(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0;
    let mut carry: u32 = 0;
    for n in 0..32 {
        let (s, c) = full_adder((a >> n) & 1, (b >> n) & 1, carry);
        carry = c;
        res |= s << n;
    }
    res
}

fn main() {
    let a = 9999;
    let b = 2352399;
    let res = adder(a, b);
    println!("Compare: {} / {}", res, a+b);
}