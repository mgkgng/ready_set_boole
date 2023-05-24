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

impl Node {
    fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf(_) => true,
            _ => false
        }
    }

    fn is_binary(&self) -> bool {
        match self {
            Node::Binary(_) => true,
            _ => false
        }
    }

    fn is_unary(&self) -> bool {
        match self {
            Node::Unary(_) => true,
            _ => false
        }
    }
}

fn get_not_node(node: Node) -> Node {
    return Node::Unary(unary_node {
        value: "!".to_string(),
        child: Some(Box::new(node))
    });
}

fn get_and_node(lhs: Node, rhs: Node) -> Node {
    return Node::Binary(binary_node {
        value: "&".to_string(),
        left: Some(Box::new(lhs)),
        right: Some(Box::new(rhs))
    });
}

fn get_or_node(lhs: Node, rhs: Node) -> Node {
    return Node::Binary(binary_node {
        value: "|".to_string(),
        left: Some(Box::new(lhs)),
        right: Some(Box::new(rhs))
    });
}

fn str_to_ast(formula: &str) -> ast_node {
    let mut stack = Vec::new();

    for c in formula.chars() {
        match c {
            c.is_uppercase() => stack.push(Node::Leaf(c.to_string())),
            '&' | '|' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                stack.push(Node::Binary(binary_node {
                    value: c.to_string(),
                    left: Some(Box::new(lhs)), 
                    right: Some(Box::new(rhs))
                }));
            },
            '^' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let not_rhs = get_not_node(rhs);
                let not_lhs = get_not_node(lhs);
                let lhs_and_not_rhs = get_and_node(lhs, not_rhs);
                let not_lhs_and_rhs = get_and_node(not_lhs, rhs);
                stack.push(get_or_node(lhs_and_not_rhs, not_lhs_and_rhs));
            },
            '>' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let not_lhs = get_not_node(lhs);
                stack.push(get_or_node(not_lhs, rhs));
            },
            '=' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let not_rhs = get_not_node(rhs);
                let not_lhs = get_not_node(lhs);
                let lhs_and_rhs = get_and_node(lhs, rhs);
                let not_lhs_and_not_rhs = get_and_node(not_lhs, not_rhs);
                stack.push(get_or_node(lhs_and_rhs, not_lhs_and_not_rhs));
            },
            '!' => {
                let val = stack.pop().expect("Invalid syntax");
                stack.push(Node::Unary(unary_node {
                    value: c.to_string(),
                    child: Some(Box::new(val))
                }));
            },
            _ => unreachable!()
        }
    }
    if stack.len() != 1 {
        panic!("Invalid syntax");
    }
    stack.pop.expect("Invalid syntax");
}

fn apply_nnf(curr: Node) -> Node {
    if curr.is_leaf() {
        return curr;
    } else if curr.is_unary() {
        if curr.child.is_leaf() {
            return curr;
        } else {
            let child = apply_nnf(curr.child);
            if child.is_leaf() {
                return get_not_node(child);
            } else if child.is_unary() {
                return child.child;
            } else if child.is_binary() {
                let lhs = apply_nnf(child.left);
                let rhs = apply_nnf(child.right);
                if child.value == "&" {
                    return get_or_node(get_not_node(lhs), get_not_node(rhs));
                } else if child.value == "|" {
                    return get_and_node(get_not_node(lhs), get_not_node(rhs));
                } else {
                    unreachable!();
                }
            }
        }
    } else {
        let lhs = apply_nnf(curr.left);
        let rhs = apply_nnf(curr.right);
        if curr.value == "&" {
            return get_and_node(lhs, rhs);
        } else if curr.value == "|" {
            return get_or_node(lhs, rhs);
        } else {
            unreachable!();
        }
    }
}

fn ast_to_str(ast: Node) -> String {
    if ast.is_leaf() {
        return ast.value;
    } else if ast.is_unary() {
        return ast_to_str(ast.child) + ast.value;
    } else if ast.is_binary() {
        return ast_to_str(ast.left) + ast_to_str(ast.right) + ast.value;
    }
}

fn negation_normal_form(formula: &str) -> String {
    let ast = str_to_ast(formula);
    let ast_nnf = apply_nnf(ast);
    let res = ast_to_str(ast_nnf);
    return res;
}

fn main() {
    let formula = "A>B&C^DvE";
    let res = negation_normal_form(formula);
    println!("{}", res);
}