fn material_condition(a: u32, b: u32) -> u32 {
    return !(a ^ b);
}

fn logical_equivalence(a: u32, b: u32) -> u32 {
    return !a | b;
}

pub fn eval_formula(formula: &str) -> bool {
    let mut stack = Vec::new();

    for c in formula.chars() {
        match c {
            '&' | '|' | '^' | '>' | '=' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let res = match c {
                    '&' => lhs & rhs,
                    '|' => lhs | rhs,
                    '^' => lhs ^ rhs,
                    '>' => material_condition(lhs, rhs),
                    '=' => logical_equivalence(lhs, rhs),
                    _ => unreachable!()
                };
                stack.push(res);
            },
            '!' => {
                let val = stack.pop().expect("Invalid syntax");
                stack.push(!val);
            },
            '0' | '1' => {
                stack.push(c.to_string().parse::<u32>().expect("Invalid syntax"));
            },
            _ => unreachable!()
        }
    }
    if stack.len() != 1 {
        panic!("Invalid syntax");
    }
    let res = stack.pop().unwrap();
    (res & 1) != 0
}

// fn main() {
//     println!("{}", eval_formula("10&")); // false
//     println!("{}", eval_formula("10|")); // true
//     println!("{}", eval_formula("11>")); // true
//     println!("{}", eval_formula("10=")); // false
//     println!("{}", eval_formula("1011||=")); // true
// }