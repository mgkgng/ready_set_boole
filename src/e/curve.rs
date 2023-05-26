fn map(x: u16, y: u16) -> f64 {
    let mut z = 0; // z gets the resulting 32-bit Morton Number.

    for i in 0..16 {
        z |= ((x as u32 & (1 << i)) << i) | ((y as u32 & (1 << i)) << (i + 1));
    }
    z as f64 / (u32::MAX as f64)
}

fn reverse_map(n: f64) -> (u16, u16) {
    if n < 0.0 || n > 1.0 {
        panic!("Invalid value: {}", n);
    }

    let z = (n * (u32::MAX as f64)) as u32;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for i in 0..16 {
        x |= ((z & (1 << 2 * i)) >> i) as u16;
        y |= ((z & (1 << (2 * i + 1))) >> (i + 1)) as u16;
    }
    (x, y)
}

// fn main() {
//     let value: f64 = 0.5; // value to be mapped

//     let (x, y) = reverse_map(value);
//     println!("Coordinates for value {}: ({}, {})", value, x, y);
//     let n = map(x, y);
//     println!("Value for coordinates ({}, {}): {}", x, y, n);
// }