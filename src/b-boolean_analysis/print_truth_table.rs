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

fn print_truth_table(formula: &str) {
    let vars = get_vars(formula);
    let vars_len = vars.len() as u32;
    let n_iter = 2_u32.pow(vars_len);

    for var in &vars { print!("| {} ", var); }
    println!("| = |");
    for _ in 0..vars_len { print!("|---"); }
    println!("|---|");
    for i in 0..n_iter {
        let mut new_form = formula.to_string();
        for (j, var) in vars.iter().enumerate() {
            let val = ((i >> (vars_len as usize - j - 1)) & 1).to_string();
            new_form = new_form.replace(&var.to_string(), &val);
            print!("| {} ", val);
        }
        println!("| {} |", eval_formula(&new_form) as u32);
    }
}

fn main() {
    print_truth_table("AB&CD&|");
}