struct binary_node {
    value: String,
    left: Option<Box<ast_node>>,
    right: Option<Box<ast_node>>
}

struct unary_node {
    value: String,
    child: Option<Box<ast_node>>
}

enum Node {
    Binary(binary_node),
    Unary(unary_node),
    Leaf(String)
}

fn str_to_ast(formula: &str) -> ast_node {
    let mut stack = Vec::new();

    for c in formula.chars() {
        match c {
            c.is_uppercase() => stack.push(Node::Leaf(c.to_string())),
            '&' | '|' | '^' | '>' | '=' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                stack.push(Node::Binary(binary_node {
                    value: c.to_string(),
                    left: Some(Box::new(lhs)), 
                    right: Some(Box::new(rhs))
                })));
            },
            '!' => {
                let val = stack.pop().expect("Invalid syntax");
                stack.push(Node::Unary(unary_node {
                    value: c.to_string(),
                    child: Some(Box::new(val))
                })));
            },
            _ => unreachable!()
        }
    }
    if stack.len() != 1 {
        panic!("Invalid syntax");
    }
    stack.pop.expect("Invalid syntax");
}

fn apply_nnf(root: Node) -> Node {
    
}

fn negation_normal_form(formula: &str) -> String {
    let ast = str_to_ast(formula);
    
}