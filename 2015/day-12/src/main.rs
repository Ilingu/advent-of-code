use std::fs;

use serde_json::Value;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let json_value: Value = serde_json::from_str(&input).unwrap();

    let p1 = traverse_json(&json_value, false).iter().sum::<i64>();
    println!("{p1}");

    let p2 = traverse_json(&json_value, true).iter().sum::<i64>();
    println!("{p2}");
}

fn traverse_json(json: &Value, is_part_2: bool) -> Vec<i64> {
    let mut res = vec![];
    match json {
        Value::Number(num) => {
            if num.is_i64() {
                res.push(num.as_i64().unwrap())
            }
        }
        Value::Array(arr) => {
            for value in arr {
                res.append(&mut traverse_json(value, is_part_2));
            }
        }
        Value::Object(obj) => {
            let mut traver_child = || {
                for (_, value) in obj {
                    res.append(&mut traverse_json(value, is_part_2));
                }
            };
            match is_part_2 {
                true => {
                    let is_ok = obj.iter().all(|(_, val)| match val {
                        Value::String(s) => !s.contains("red"),
                        _ => true,
                    });
                    if is_ok {
                        traver_child()
                    }
                }
                false => traver_child(),
            }
        }
        _ => (),
    };
    res
}
