use std::collections::HashSet;

fn material_condition(a: u32, b: u32) -> u32 {
    return !a | b;
}

fn logical_equivalence(a: u32, b: u32) -> u32 {
    return !(a ^ b);
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

fn get_vars(formula: &str) -> Vec<char> {
    let mut vars_set = HashSet::new();
    let ops = "!&|^>=";

    for c in formula.chars() {
        if c.is_uppercase() {
            vars_set.insert(c);
        } else if !ops.contains(c) {
            panic!("Error: Character {} not allowed", c);
        }
    }

    let mut vars: Vec<char> = vars_set.into_iter().collect();
    vars.sort();
    return vars;
}

fn sat(formula: &str) -> bool {
    let vars = get_vars(formula);
    let vars_len = vars.len() as u32;
    let n_iter = 2_u32.pow(vars_len);

    for i in 0..n_iter {
        let mut new_form = formula.to_string();
        for (j, var) in vars.iter().enumerate() {
            let val = ((i / 2_u32.pow(vars_len - 1 - (j as u32))) % 2).to_string();
            new_form = new_form.replace(&var.to_string(), &val);
        }
        if eval_formula(&new_form) == true {
            return true;
        }
    }
    return false;
}

// fn main() {
//     println!("{}", sat("AB|")); // true
//     println!("{}", sat("AB&")); // true
//     println!("{}", sat("AA!&")); // false
//     println!("{}", sat("AA^")); // false
// }