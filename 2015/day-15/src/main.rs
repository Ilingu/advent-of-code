use evalexpr::eval_int;

const INPUT: &str = "Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
Candy: capacity 0, durability 5, flavor -1, texture 0, calories 8
Butterscotch: capacity -1, durability 0, flavor 5, texture 0, calories 6
Sugar: capacity 0, durability 0, flavor -2, texture 2, calories 1";

fn main() {
    let cookie_ingredients = INPUT
        .lines()
        .map(CookieIngredient::from_input)
        .collect::<Vec<_>>();
    let p1 = cookie_ingredients.best_recipe_score(None);
    println!("{p1}");

    let p2 = cookie_ingredients.best_recipe_score(Some(500));
    println!("{p2}");
}

struct CookieIngredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl CookieIngredient {
    fn from_input(line: &str) -> Self {
        let datas = line.split_whitespace().collect::<Vec<_>>();
        Self {
            capacity: datas[2].trim_matches(',').parse::<isize>().unwrap(),
            durability: datas[4].trim_matches(',').parse::<isize>().unwrap(),
            flavor: datas[6].trim_matches(',').parse::<isize>().unwrap(),
            texture: datas[8].trim_matches(',').parse::<isize>().unwrap(),
            calories: datas[10].trim_matches(',').parse::<isize>().unwrap(),
        }
    }
}

const VARIABLES: &str = "abcdefghijklmnopqrstuvwxyz";

trait CookieMaker {
    fn best_recipe_score(&self, calories_limit: Option<usize>) -> usize;
}

impl CookieMaker for Vec<CookieIngredient> {
    fn best_recipe_score(&self, calories_limit: Option<usize>) -> usize {
        let vars = VARIABLES.chars().collect::<Vec<_>>();

        let build_inner_expr = |map: Vec<isize>| {
            map.iter()
                .enumerate()
                .map(|(i, d)| format!("({}*({}))", d, vars[i]))
                .collect::<Vec<_>>()
                .join("+")
        };

        let capacity = build_inner_expr(self.iter().map(|ci| ci.capacity).collect());
        let durability = build_inner_expr(self.iter().map(|ci| ci.durability).collect());
        let flavor = build_inner_expr(self.iter().map(|ci| ci.flavor).collect());
        let texture = build_inner_expr(self.iter().map(|ci| ci.texture).collect());
        let calories = build_inner_expr(self.iter().map(|ci| ci.calories).collect());

        let replace_with_value = |expr: &String, values: &Vec<(char, String)>| {
            let mut new_expr = expr.clone();
            for (ch, val) in values {
                new_expr = new_expr.replace(*ch, val.as_str())
            }
            new_expr
        };

        let mut max_total = 0;
        for a in 0..=100 {
            for b in 0..=(100 - a) {
                for c in 0..=(100 - a - b) {
                    let d = 100 - a - b - c;
                    let values = vec![
                        ('a', a.to_string()),
                        ('b', b.to_string()),
                        ('c', c.to_string()),
                        ('d', d.to_string()),
                    ];

                    if let Some(limit) = calories_limit {
                        let calories_score =
                            eval_int(&replace_with_value(&calories, &values)).unwrap();
                        if calories_score != limit as i64 {
                            continue;
                        }
                    }

                    let capacity_score = eval_int(&replace_with_value(&capacity, &values))
                        .unwrap()
                        .pos_or_0();
                    let durability_score = eval_int(&replace_with_value(&durability, &values))
                        .unwrap()
                        .pos_or_0();
                    let flavor_score = eval_int(&replace_with_value(&flavor, &values))
                        .unwrap()
                        .pos_or_0();
                    let texture_score = eval_int(&replace_with_value(&texture, &values))
                        .unwrap()
                        .pos_or_0();

                    let total_score =
                        capacity_score * durability_score * flavor_score * texture_score;
                    max_total = max_total.max(total_score);
                }
            }
        }
        max_total
    }
}

trait PosOr0 {
    fn pos_or_0(self) -> usize;
}

impl PosOr0 for i64 {
    fn pos_or_0(self) -> usize {
        if self < 0 {
            0
        } else {
            self as usize
        }
    }
}
