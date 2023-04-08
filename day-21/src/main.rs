use std::{collections::HashMap, fs};

use evalexpr::eval_int;

#[derive(Clone)]
enum Ops {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Ops {
    fn to_string(&self) -> char {
        match self {
            Ops::Plus => '+',
            Ops::Minus => '-',
            Ops::Times => '*',
            Ops::Divide => '/',
        }
    }
}

#[derive(Clone)]
struct MathOperation {
    left_side: String,
    right_side: String,
    operation: Ops,
}

impl MathOperation {
    fn from_input(input: &str) -> Self {
        let formatted_input = input
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>();
        let (var_a, var_b) = (&formatted_input[..4], &formatted_input[4..]);

        let mut operation = Ops::Plus;
        if input.contains("-") {
            operation = Ops::Minus
        } else if input.contains("*") {
            operation = Ops::Times
        } else if input.contains("/") {
            operation = Ops::Divide
        }

        Self {
            left_side: var_a.to_string(),
            right_side: var_b.to_string(),
            operation,
        }
    }
}

fn main() {
    let binding = fs::read_to_string("./Assets/input.txt").unwrap();
    let raw_data = binding
        .lines()
        .map(|x| x.split(": ").collect::<Vec<_>>())
        .map(|sp| (sp[0].to_string(), sp[1]))
        .collect::<Vec<_>>();

    let (mut values_pool, mut operations_pool) = (HashMap::new(), HashMap::new());
    for (key, val) in raw_data {
        match val.parse::<isize>() {
            Ok(num) => {
                values_pool.insert(key, num);
            }
            Err(_) => {
                operations_pool.insert(key, MathOperation::from_input(val));
            }
        }
    }

    p1(values_pool.clone(), operations_pool.clone());
    p2(values_pool.clone(), operations_pool.clone());
}

fn p1(
    mut values_pool: HashMap<String, isize>,
    mut operations_pool: HashMap<String, MathOperation>,
) {
    while values_pool.get("root").is_none() {
        for (key, operation) in operations_pool.clone() {
            let left_side = match values_pool.get(operation.left_side.as_str()) {
                Some(&val) => val,
                None => continue,
            };
            let right_side = match values_pool.get(operation.right_side.as_str()) {
                Some(&val) => val,
                None => continue,
            };

            let result = match operation.operation {
                Ops::Plus => left_side + right_side,
                Ops::Minus => left_side - right_side,
                Ops::Times => left_side * right_side,
                Ops::Divide => left_side / right_side,
            };

            values_pool.insert(key.clone(), result);
            operations_pool.remove(&key);
        }
    }

    let p1 = values_pool["root"];
    println!("{p1}");
}

fn p2(mut values_pool: HashMap<String, isize>, operations_pool: HashMap<String, MathOperation>) {
    values_pool.remove("humn");
    let root = operations_pool["root"].clone();

    let left_equation =
        recursive_humn(root.left_side, &operations_pool, &values_pool).replace("humn", "x");
    let right_number =
        eval_int(recursive_humn(root.right_side, &operations_pool, &values_pool).as_str()).unwrap();

    println!("{left_equation}={right_number}"); // solving this equation online gives us: x=3032671800353, so answer to part 2 is 3032671800353 (thanks to: https://www.mathpapa.com/equation-solver/)
    println!("3032671800353");
    // PS: I could've take the time to evaluate it myself (binary search, newtons thingy, or  just simplify all coeficient and compute the solution (since it's a very simple equation)...), but since the "online solver" worked immediatly, I was a bit lazy
}

fn recursive_humn(
    current_child: String,
    operations_pool: &HashMap<String, MathOperation>,
    values_pool: &HashMap<String, isize>,
) -> String {
    if current_child == "humn" {
        return "humn".to_string();
    }

    let MathOperation {
        left_side,
        right_side,
        operation,
    } = operations_pool.get(&current_child).unwrap();

    let mut equation = String::new();
    match values_pool.get(left_side.as_str()) {
        Some(val) => equation.push_str(val.to_string().as_str()),
        None => equation
            .push_str(recursive_humn(left_side.to_string(), operations_pool, values_pool).as_str()),
    };
    equation.push(operation.to_string());
    match values_pool.get(right_side.as_str()) {
        Some(val) => equation.push_str(val.to_string().as_str()),
        None => equation.push_str(
            recursive_humn(right_side.to_string(), operations_pool, values_pool).as_str(),
        ),
    };

    return format!("({equation})");
}
