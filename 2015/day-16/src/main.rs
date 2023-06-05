use std::fs;

#[derive(Debug, Copy, Clone)]
struct Mfcsam {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Mfcsam {
    fn default() -> Self {
        Self {
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn new_from_line(line: &str) -> Self {
        let skip = line.find(|c| c == ':').unwrap();
        let datas_str = &line[skip + 2..];
        let datas = datas_str
            .split(", ")
            .map(|kv| {
                let mut kv_data = kv.split(": ");
                (kv_data.next().unwrap(), kv_data.next().unwrap())
            })
            .collect::<Vec<_>>();

        let mut parsed_data = Self::default();
        for (key, val) in datas {
            match key {
                "children" => parsed_data.children = Some(val.parse::<usize>().unwrap()),
                "cats" => parsed_data.cats = Some(val.parse::<usize>().unwrap()),
                "samoyeds" => parsed_data.samoyeds = Some(val.parse::<usize>().unwrap()),
                "pomeranians" => parsed_data.pomeranians = Some(val.parse::<usize>().unwrap()),
                "akitas" => parsed_data.akitas = Some(val.parse::<usize>().unwrap()),
                "vizslas" => parsed_data.vizslas = Some(val.parse::<usize>().unwrap()),
                "goldfish" => parsed_data.goldfish = Some(val.parse::<usize>().unwrap()),
                "trees" => parsed_data.trees = Some(val.parse::<usize>().unwrap()),
                "cars" => parsed_data.cars = Some(val.parse::<usize>().unwrap()),
                "perfumes" => parsed_data.perfumes = Some(val.parse::<usize>().unwrap()),
                _ => unreachable!("{:?}", (key, val)),
            }
        }
        parsed_data
    }

    fn partial_match(&self, comparer: &Self, is_part_1: bool) -> bool {
        let comparer_vec = comparer.into_iter().collect::<Vec<_>>();
        let mut does_match = true;
        for (i, (k, v)) in self.into_iter().enumerate() {
            let (_, cv) = comparer_vec[i];
            // assert_eq!(k.clone(), ck.clone());
            does_match = does_match
                && match is_part_1 {
                    true => v.is_none() || cv.is_none() || v == cv,
                    false => match k.as_str() {
                        "cats" | "trees" => v.is_none() || cv.is_none() || v.unwrap() > cv.unwrap(),
                        "pomeranians" | "goldfish" => {
                            v.is_none() || cv.is_none() || v.unwrap() < cv.unwrap()
                        }
                        _ => v.is_none() || cv.is_none() || v == cv,
                    },
                }
        }

        does_match
    }
}

impl IntoIterator for Mfcsam {
    type Item = (String, Option<usize>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("children".to_string(), self.children),
            ("cats".to_string(), self.cats),
            ("samoyeds".to_string(), self.samoyeds),
            ("pomeranians".to_string(), self.pomeranians),
            ("akitas".to_string(), self.akitas),
            ("vizslas".to_string(), self.vizslas),
            ("goldfish".to_string(), self.goldfish),
            ("trees".to_string(), self.trees),
            ("cars".to_string(), self.cars),
            ("perfumes".to_string(), self.perfumes),
        ]
        .into_iter()
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let original_aunt_sue = Mfcsam {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let sue_aunts = input
        .lines()
        .enumerate()
        .map(|(id, line)| (id + 1, Mfcsam::new_from_line(line)))
        .collect::<Vec<_>>();

    {
        let best_matches_sue = sue_aunts
            .iter()
            .filter(|(_, mfcsam)| mfcsam.partial_match(&original_aunt_sue, true))
            .collect::<Vec<_>>();
        assert_eq!(best_matches_sue.len(), 1);
        let (p1, _) = best_matches_sue[0];
        println!("{p1}");
    }

    {
        let best_matches_sue = sue_aunts
            .iter()
            .filter(|(_, mfcsam)| mfcsam.partial_match(&original_aunt_sue, false))
            .collect::<Vec<_>>();
        assert_eq!(best_matches_sue.len(), 1);
        let (p2, _) = best_matches_sue[0];
        println!("{p2}");
    }
}
