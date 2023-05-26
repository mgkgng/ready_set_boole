use std::collections::HashSet;

fn get_complement(set: &Vec<i32>, universal: &HashSet<i32>) -> Vec<i32> {
    let set: HashSet<i32> = set.iter().cloned().collect();
    universal.iter().cloned().filter(|i| !set.contains(i)).collect()
}

fn get_union(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = set1.iter().cloned().collect();
    let set2: HashSet<i32> = set2.iter().cloned().collect();
    set1.union(&set2).cloned().collect()
}

fn get_intersection(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = set1.iter().cloned().collect();
    let set2: HashSet<i32> = set2.iter().cloned().collect();
    set1.intersection(&set2).cloned().collect()
}

fn get_implication(set1: &Vec<i32>, set2: &Vec<i32>, universal: &HashSet<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = set1.iter().cloned().collect();
    let set2: HashSet<i32> = set2.iter().cloned().collect();
    let universal: HashSet<i32> = universal.iter().cloned().collect();
    
    let not_set1 = universal.difference(&set1).cloned().collect::<HashSet<_>>();
    
    not_set1.union(&set2).cloned().collect()
}

fn get_symmetric_difference(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = set1.iter().cloned().collect();
    let set2: HashSet<i32> = set2.iter().cloned().collect();
    set1.symmetric_difference(&set2).cloned().collect()
}

fn get_logical_equivalence(set1: &Vec<i32>, set2: &Vec<i32>, universal: &HashSet<i32>) -> Vec<i32> {
    let intersection: HashSet<i32> = get_intersection(set1, set2).into_iter().collect();
    let uni = get_union(set1, set2);
    let not_union: HashSet<i32> = get_complement(&uni, universal).into_iter().collect();
    intersection.union(&not_union).cloned().collect()
}

fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let universal: HashSet<i32> = sets.iter().flatten().cloned().collect();

    let mut stack = Vec::new();
    for c in formula.chars() {
        match c {
            '&' | '|' | '^' | '>' | '=' => {
                let set2 = stack.pop().expect("Invalid syntax");
                let set1 = stack.pop().expect("Invalid syntax");
                let res = match c {
                    '&' => get_intersection(&set1, &set2),
                    '|' => get_union(&set1, &set2),
                    '^' => get_symmetric_difference(&set1, &set2),
                    '>' => get_implication(&set1, &set2, &universal),
                    '=' => get_logical_equivalence(&set1, &set2, &universal),
                    _ => unreachable!()
                };
                stack.push(res);
            },
            '!' => {
                let set = stack.pop().expect("Invalid syntax");
                stack.push(get_complement(&set, &universal));
            },
            _ if c.is_uppercase() => {
                let index: u8 = (c as u8) - ('A' as u8);
                if index >= sets.len() as u8 {
                    panic!("Invalid syntax");
                }
                stack.push(sets[index as usize].clone());
            },
            _ => unreachable!()
        }
    }
    if stack.len() != 1 {
        panic!("Invalid syntax");
    }
    let mut res = stack.pop().unwrap();
    res.sort();
    res
}

fn main() {
    let sets = vec![
        vec![0, 1, 2],
        vec![0, 3, 4],
    ];
    let result = eval_set("AB&", &sets); // [0]
    println!("{:?}", result);

    let sets = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
    ];
    let result = eval_set("AB|", &sets); // [0, 1, 2, 3, 4, 5]
    println!("{:?}", result);

    let sets = vec![
        vec![0, 1, 2],
    ];
    let result = eval_set("A!", &sets); // []
    println!("{:?}", result);

}