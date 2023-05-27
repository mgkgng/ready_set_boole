#[derive(Clone)]
pub struct BinaryNode {
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

#[derive(Clone)]
pub struct UnaryNode {
    value: String,
    child: Option<Box<Node>>
}

#[derive(Clone)]
pub struct LeafNode {
    value: String
}

#[derive(Clone)]
pub enum Node {
    Binary(BinaryNode),
    Unary(UnaryNode),
    Leaf(LeafNode)
}

fn get_not_node(node: &Node) -> Node {
    return Node::Unary(UnaryNode {
        value: "!".to_string(),
        child: Some(Box::new(node.clone()))
    });
}

fn get_and_node(lhs: &Node, rhs: &Node) -> Node {
    return Node::Binary(BinaryNode {
        value: "&".to_string(),
        left: Some(Box::new(lhs.clone())),
        right: Some(Box::new(rhs.clone()))
    });
}

fn get_or_node(lhs: &Node, rhs: &Node) -> Node {
    return Node::Binary(BinaryNode {
        value: "|".to_string(),
        left: Some(Box::new(lhs.clone())),
        right: Some(Box::new(rhs.clone()))
    });
}

fn get_conjunctive_distribution(node: &Node) -> Node {
    match node {
        Node::Binary(binary) if binary.value == "|" => {
            let left = binary.left.clone().expect("Invalid syntax");
            let right = binary.right.clone().expect("Invalid syntax");

            match (&*left, &*right) {
                (Node::Binary(binary_left), _) if binary_left.value == "&" =>
                    Node::Binary(BinaryNode {
                        value: "&".to_string(),
                        left: Some(Box::new(get_conjunctive_distribution(&get_or_node(&binary_left.left.as_ref().unwrap(), &right)))),
                        right: Some(Box::new(get_conjunctive_distribution(&get_or_node(&binary_left.right.as_ref().unwrap(), &right))))
                    }),
                (_, Node::Binary(binary_right)) if binary_right.value == "&" =>
                    Node::Binary(BinaryNode {
                        value: "&".to_string(),
                        left: Some(Box::new(get_conjunctive_distribution(&get_or_node(&left, &binary_right.left.as_ref().unwrap())))),
                        right: Some(Box::new(get_conjunctive_distribution(&get_or_node(&left, &binary_right.right.as_ref().unwrap()))))
                    }),
                _ => node.clone()
            }
        },
        _ => node.clone()
    }
}

fn str_to_ast(formula: &str) -> Node {
    let mut stack = Vec::new();

    for c in formula.chars() {
        match c {
            _ if c.is_uppercase() => stack.push(Node::Leaf(LeafNode { value: c.to_string() })),
            '&' | '|' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                stack.push(Node::Binary(BinaryNode {
                    value: c.to_string(),
                    left: Some(Box::new(lhs)), 
                    right: Some(Box::new(rhs))
                }));
            },
            '^' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let not_rhs = get_not_node(&rhs);
                let not_lhs = get_not_node(&lhs);
                let lhs_and_not_rhs = get_and_node(&lhs, &not_rhs);
                let not_lhs_and_rhs = get_and_node(&not_lhs, &rhs);
                stack.push(get_or_node(&lhs_and_not_rhs, &not_lhs_and_rhs));
            },
            '>' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let not_lhs = get_not_node(&lhs);
                stack.push(get_or_node(&not_lhs, &rhs));
            },
            '=' => {
                let rhs = stack.pop().expect("Invalid syntax");
                let lhs = stack.pop().expect("Invalid syntax");
                let not_rhs = get_not_node(&rhs);
                let not_lhs = get_not_node(&lhs);
                let lhs_and_rhs = get_and_node(&lhs, &rhs);
                let not_lhs_and_not_rhs = get_and_node(&not_lhs, &not_rhs);
                stack.push(get_or_node(&lhs_and_rhs, &not_lhs_and_not_rhs));
            },
            '!' => {
                let val = stack.pop().expect("Invalid syntax");
                stack.push(Node::Unary(UnaryNode {
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
    let root = stack.pop().expect("Invalid syntax");
    return root;
}

fn apply_nnf(curr: Node) -> Node {
    match curr {
        Node::Leaf(leaf) => {
            Node::Leaf(leaf)
        },
        Node::Binary(binary) => {
            let left = binary.left.expect("Invalid syntax");
            let right = binary.right.expect("Invalid syntax");
            let left_nnf = apply_nnf(*left);
            let right_nnf = apply_nnf(*right);
            Node::Binary(BinaryNode {
                value: binary.value,
                left: Some(Box::new(left_nnf)),
                right: Some(Box::new(right_nnf))
            })
        },
        Node::Unary(unary) => {
            let child = unary.child.expect("Invalid syntax");
            match *child {
                Node::Binary(binary) => {
                    let left = binary.left.expect("Invalid syntax");
                    let right = binary.right.expect("Invalid syntax");
                    let left_nnf = apply_nnf(*left);
                    let right_nnf = apply_nnf(*right);
        
                    if binary.value == "&" {
                        let left_nnf_not = get_not_node(&left_nnf);
                        let right_nnf_not = get_not_node(&right_nnf);
                        let left_nnf_not_nnf = apply_nnf(left_nnf_not);
                        let right_nnf_not_nnf = apply_nnf(right_nnf_not);
                        return get_or_node(&left_nnf_not_nnf, &right_nnf_not_nnf);
                    } else if binary.value == "|" {
                        let left_nnf_not = get_not_node(&left_nnf);
                        let right_nnf_not = get_not_node(&right_nnf);
                        let left_nnf_not_nnf = apply_nnf(left_nnf_not);
                        let right_nnf_not_nnf = apply_nnf(right_nnf_not);
                        return get_and_node(&left_nnf_not_nnf, &right_nnf_not_nnf);
                    } else {
                        unreachable!();
                    }
                },
                Node::Unary(unary_child) => {
                    return apply_nnf(*unary_child.child.expect("Invalid syntax"));
                },
                _ => {
                    return get_not_node(&apply_nnf(*child));
                }
            }
        }
    }
}

fn apply_cnf(curr: Node) -> Node {
    match curr {
        Node::Binary(binary) => {
            let left = binary.left.clone().expect("Invalid syntax");
            let right = binary.right.clone().expect("Invalid syntax");
            let left_cnf = apply_cnf(*left);
            let right_cnf = apply_cnf(*right);

            if binary.value == "|" {
                return get_conjunctive_distribution(&Node::Binary(BinaryNode {
                    value: "|".to_string(),
                    left: Some(Box::new(left_cnf)),
                    right: Some(Box::new(right_cnf))
                }));
            } else {
                Node::Binary(BinaryNode {
                    value: binary.value.to_string(),
                    left: Some(Box::new(left_cnf)),
                    right: Some(Box::new(right_cnf))
                })
            }
        },
        Node::Unary(unary) => {
            let child = unary.child.clone().expect("Invalid syntax");
            let child_cnf = apply_cnf(*child);
            Node::Unary(UnaryNode {
                value: unary.value.to_string(),
                child: Some(Box::new(child_cnf))
            })
        },
        _ => curr
    }
}

fn ast_to_str(curr: Node) -> String {
    let mut res: String = "".to_string();
    match curr {
        Node::Leaf(leaf) => {
            res += &leaf.value;
        },
        Node::Binary(binary) => {
            let op = binary.value.chars().nth(0);
            res += &binary.value;
            let left_str = &ast_to_str(*binary.right.expect("Invalid syntax"));
            let mut left_done = false;
            let right_str = &ast_to_str(*binary.left.expect("Invalid syntax"));
            let mut right_done = false;
            if op == right_str.chars().nth(0) {
                res += right_str;
                right_done = true;
            }
            if op == left_str.chars().nth(0) {
                let (binary, body) = left_str.split_at(1);
                let new_res = binary.to_string() + &res + &body.to_string(); 
                res = new_res;
                left_done = true;
            }
            if !left_done {
                res += &left_str;
            }
            if !right_done {
                res += &right_str;
            }
        },
        Node::Unary(unary) => {
            res += &unary.value;
            res += &ast_to_str(*unary.child.expect("Invalid syntax"));
        }
    }
    return res;
}

fn conjunctive_normal_form(formula: &str) -> String {
    let ast = str_to_ast(formula);
    let ast_nnf = apply_nnf(ast);
    let ast_cnf = apply_cnf(ast_nnf);
    let res = ast_to_str(ast_cnf);
    return res.chars().rev().collect::<String>();
}

// fn main() {
//     println!("{}", conjunctive_normal_form("AB&!")); // A!B!|
//     println!("{}", conjunctive_normal_form("AB|!")); // A!B!&
//     println!("{}", conjunctive_normal_form("AB|C&")); // AB|C&
//     println!("{}", conjunctive_normal_form("AB|C|D|")); // ABCD|||
//     println!("{}", conjunctive_normal_form("AB&C&D&")); // ABCD&&&
//     println!("{}", conjunctive_normal_form("AB&!C!|")); // A!B!C!||
//     println!("{}", conjunctive_normal_form("AB|!C!&")); // A!B!C!&&
// }