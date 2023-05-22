fn eval_formula(formula: &str) -> bool {
    for c in formula.chars() {
        println!("cc {}", c);
    }
    return true;
}

fn main() {
    let res = eval_formula("10|1&");
}