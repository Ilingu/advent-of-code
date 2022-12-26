#[derive(Debug, Clone, Copy)]
enum Signs {
    PLUS,
    TIMES,
}

impl Signs {
    pub fn parse(str_to_parse: &str) -> Self {
        match str_to_parse {
            "+" => Self::PLUS,
            "*" => Self::TIMES,
            _ => panic!("invalid op"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    left: String,
    right: String,
    sign: Signs,
}

impl Operation {
    pub fn parse(str_to_parse: &str) -> Self {
        let mut new_expr = str_to_parse
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .trim()
            .trim_start_matches("new = ")
            .split_whitespace();
        assert_eq!(new_expr.clone().count(), 3);

        return Self {
            left: new_expr.next().unwrap().to_string(),
            sign: Signs::parse(new_expr.next().unwrap()),
            right: new_expr.next().unwrap().to_string(),
        };
    }

    pub fn exec(&self, num: i32) -> i32 {
        let parse_rl = |rl: &String| rl.replace("old", &num.to_string()).parse::<i32>().unwrap();
        match self.sign {
            Signs::PLUS => parse_rl(&self.left) + parse_rl(&self.right),
            Signs::TIMES => parse_rl(&self.left) * parse_rl(&self.right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: usize,
    pub items_worry: Vec<i32>,
    pub op: Operation,
    pub test: i32,
    pub mk_true: usize,
    pub mk_false: usize,
    pub inspected_no: usize,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            id: 0,
            items_worry: vec![],
            op: Operation {
                left: "".to_string(),
                right: "".to_string(),
                sign: Signs::PLUS,
            },
            test: 0,
            mk_true: 0,
            mk_false: 0,
            inspected_no: 0,
        }
    }

    pub fn from_string(string_to_parse: &str) -> Self {
        let mut datas = string_to_parse.lines();
        assert_eq!(datas.clone().count(), 6);

        let parse_numerics = |txt: &str| {
            txt.matches(char::is_numeric)
                .collect::<Vec<&str>>()
                .join("")
                .parse::<i32>()
                .unwrap()
        };

        let mk_id = parse_numerics(datas.next().unwrap()) as usize;

        let items_worry = datas
            .next()
            .unwrap()
            .replace(" ", "")
            .trim_start_matches("Startingitems:")
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let op = Operation::parse(datas.next().unwrap().trim());

        let mk_test = parse_numerics(datas.next().unwrap().trim());
        let mk_true = parse_numerics(datas.next().unwrap().trim()) as usize;
        let mk_false = parse_numerics(datas.next().unwrap().trim()) as usize;

        return Self {
            id: mk_id,
            items_worry,
            op,
            test: mk_test,
            mk_true,
            mk_false,
            inspected_no: 0,
        };
    }

    /// return the new mk_id to throw at
    pub fn inspect(&mut self, item_id: usize) -> (usize, i32) {
        self.inspected_no += 1;
        let mut worry_lvl = *self.items_worry.get(item_id).unwrap();
        worry_lvl = self.op.exec(worry_lvl);
        worry_lvl = worry_lvl.div_euclid(3);

        if worry_lvl % self.test == 0 {
            return (self.mk_true, worry_lvl);
        }
        return (self.mk_false, worry_lvl);
    }
}
