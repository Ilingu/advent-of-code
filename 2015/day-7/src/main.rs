use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Operators {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    PASS,
}

impl Operators {
    fn from_str(input: &str) -> Self {
        match input {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "LSHIFT" => Self::LSHIFT,
            "RSHIFT" => Self::RSHIFT,
            "NOT" => Self::NOT,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Operation<'a> {
    operator: Operators,
    in_wires: (&'a str, Option<&'a str>),
    out_wire: &'a str,
}

impl Operation<'_> {
    fn exec(opool: &Vec<Operation>, vpool: &HashMap<&str, isize>) -> isize {
        let mut ops_pool = opool.clone();
        let mut values_pool = vpool.clone();

        while !ops_pool.is_empty() {
            let mut idx_to_remove = vec![];
            for (
                i,
                Operation {
                    operator,
                    in_wires,
                    out_wire,
                },
            ) in ops_pool.iter().enumerate()
            {
                let (left_value, right_value): (isize, isize);
                match in_wires.0.parse::<isize>() {
                    Ok(val) => left_value = val,
                    Err(_) => {
                        let val = values_pool.get(in_wires.0);
                        if val.is_none() {
                            continue;
                        }
                        left_value = *val.unwrap();
                    }
                };
                match operator {
                    Operators::AND | Operators::OR | Operators::LSHIFT | Operators::RSHIFT => {
                        match in_wires.1.unwrap().parse::<isize>() {
                            Ok(val) => right_value = val,
                            Err(_) => {
                                let val = values_pool.get(in_wires.1.unwrap());
                                if val.is_none() {
                                    continue;
                                }
                                right_value = *val.unwrap();
                            }
                        };

                        let out_value: isize;
                        match operator {
                            Operators::AND => {
                                out_value = (left_value & right_value).rem_euclid(65536)
                            }
                            Operators::OR => {
                                out_value = (left_value | right_value).rem_euclid(65536)
                            }
                            Operators::LSHIFT => {
                                out_value = (left_value << right_value).rem_euclid(65536)
                            }
                            Operators::RSHIFT => {
                                out_value = (left_value >> right_value).rem_euclid(65536)
                            }
                            _ => continue,
                        }
                        values_pool.insert(&out_wire, out_value);
                        idx_to_remove.push(i);
                    }
                    Operators::NOT => {
                        let out_value = (!left_value).rem_euclid(65536);
                        values_pool.insert(&out_wire, out_value);
                        idx_to_remove.push(i);
                    }
                    Operators::PASS => {
                        values_pool.insert(&out_wire, left_value);
                        idx_to_remove.push(i);
                    }
                }
            }

            for i in idx_to_remove.iter().rev() {
                ops_pool.remove(*i);
            }
        }
        values_pool["a"]
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut initial_ops_pool = vec![];
    let mut initial_values_pool = HashMap::new();
    for instruction in input.lines() {
        let datas = instruction.split_whitespace().collect::<Vec<_>>();

        match datas.len() {
            3 => {
                let (value, out_wire) = (datas[0], datas[2]);
                if value.parse::<isize>().is_ok() {
                    initial_values_pool.insert(out_wire, value.parse::<isize>().unwrap());
                } else {
                    let op = Operation {
                        operator: Operators::PASS,
                        in_wires: (value, None),
                        out_wire,
                    };
                    initial_ops_pool.push(op);
                }
            }
            4 => {
                let (in_wire, out_wire) = (datas[1], datas[3]);
                let op = Operation {
                    operator: Operators::NOT,
                    in_wires: (in_wire, None),
                    out_wire,
                };
                initial_ops_pool.push(op);
            }
            5 => {
                let (left_wire, operator, right_wire, out_wire) =
                    (datas[0], datas[1], datas[2], datas[4]);
                let op = Operation {
                    operator: Operators::from_str(operator),
                    in_wires: (left_wire, Some(right_wire)),
                    out_wire,
                };
                initial_ops_pool.push(op);
            }
            _ => panic!("{instruction}"),
        };
    }

    let p1 = Operation::exec(&initial_ops_pool, &initial_values_pool);
    println!("{p1}");

    initial_values_pool.insert("b", p1);
    let p2 = Operation::exec(&initial_ops_pool, &initial_values_pool);
    println!("{p2}");
}
