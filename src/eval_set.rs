fn eval_set(formula: &str, sets: &[[i32]]) -> Vec<i32> {
    let mut stack = Vec::new();
    let universal = sets.iter().fold(Vec::new(), |mut acc, set| {
        acc.extend(set);
        acc
    });

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
                stack.push(c.to_string().parse::<u32>().expect("COUCOUCOU"));
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