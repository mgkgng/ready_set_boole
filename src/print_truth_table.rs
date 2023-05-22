// mod eval_formula;
// use eval_formula::eval_formula;
use std::collections::HashSet;
use std::collections::HashMap;

fn print_truth_table(formula: &str) {
    let mut vars_set = HashSet::new();
    let ops = "!&|>=";

    for c in formula.chars() {
        if c.is_uppercase() {
            vars_set.insert(c);
        } else if !ops.contains(c) {
            panic!("Error: Character {} not allowed", c);
        }
    }

    let mut vars: Vec<char> = vars_set.into_iter().collect();
    vars.sort();

    let vars_len = vars.len() as u32;
    let n_iter = 2_u32.pow(vars_len);
    // let mut expression = Vec::new();
    for i in 0..n_iter {
        let mut val_pair: HashMap<char, char> = HashMap::new();
        let mut new_form = formula.to_string();
        for (j, var) in vars.iter().enumerate() {
            let val = ((i / 2_u32.pow(vars_len - 1 - (j as u32))) % 2).to_string();
            new_form = new_form.replace(&var.to_string(), &val);
        }
        println!("{}th iter: {}", i, new_form);
    }
}

fn main() {
    print_truth_table("AB&C|");
}