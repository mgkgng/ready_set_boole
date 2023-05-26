mod eval_formula;
use eval_formula::eval_formula;
use std::collections::HashSet;

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

fn main() {
    println!("{}", sat("AB|")); // true
    println!("{}", sat("AB&")); // true
    println!("{}", sat("AA!&")); // false
    println!("{}", sat("AA^")); // false
}