#[derive(Clone)]
struct BinaryNode {
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

#[derive(Clone)]
struct UnaryNode {
    value: String,
    child: Option<Box<Node>>
}

#[derive(Clone)]
struct LeafNode {
    value: String
}

#[derive(Clone)]
enum Node {
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
            let left = binary.left.map(|box_node| apply_nnf(*box_node));
            let right = binary.right.map(|box_node| apply_nnf(*box_node));
            Node::Binary(BinaryNode {
                value: binary.value,
                left: left.map(Box::new),
                right: right.map(Box::new),
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
        
                    // let left = binary.left.map(|box_node| apply_nnf(*box_node));
                    // let right = binary.right.map(|box_node| apply_nnf(*box_node));
                    // let lhs = apply_nnf(*binary.left.expect("Invalid syntax"));
                    // let rhs = apply_nnf(*binary.right.expect("Invalid syntax"));
                    if binary.value == "&" {
                        return get_or_node(&get_not_node(&left_nnf), &get_not_node(&right_nnf));
                    } else if binary.value == "|" {
                        return get_and_node(&get_not_node(&left_nnf), &get_not_node(&right_nnf));
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

fn ast_to_str(curr: Node) -> String {
    let mut res: String = "".to_string();
    match curr {
        Node::Leaf(leaf) => {
            res += &leaf.value;
        },
        Node::Binary(binary) => {
            res += &binary.value;
            res += &ast_to_str(*binary.right.expect("Invalid syntax"));
            res += &ast_to_str(*binary.left.expect("Invalid syntax"));
        },
        Node::Unary(unary) => {
            res += &ast_to_str(*unary.child.expect("Invalid syntax"));
            res += &unary.value;
        }
    }
    return res.chars().rev().collect::<String>();
}

fn negation_normal_form(formula: &str) -> String {
    let ast = str_to_ast(formula);
    let ast_nnf = apply_nnf(ast);
    let res = ast_to_str(ast_nnf);
    return res;
}

fn main() {
    println!("{}", negation_normal_form("A!!")); // A
    println!("{}", negation_normal_form("AB&!")); // A!B!|
    println!("{}", negation_normal_form("AB|!")); // A!B!&
    println!("{}", negation_normal_form("AB>")); // A!B|
    println!("{}", negation_normal_form("AB=")); // AB&A!B!&|
    println!("{}", negation_normal_form("AB|C&!")); // A!B!&C!|
}