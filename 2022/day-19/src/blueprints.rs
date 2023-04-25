use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct CostInfo {
    value: usize,
    material: Material,
}

impl CostInfo {
    pub fn new(value: usize, material: Material) -> Self {
        Self { value, material }
    }
}

#[derive(Debug)]
pub struct Blueprints {
    pub id: usize,
    costs: HashMap<Material, Vec<CostInfo>>,
    robots: HashMap<Material, usize>,
    treasury: HashMap<Material, usize>,
}

impl Blueprints {
    pub fn from_input(raw: &str) -> Self {
        let re = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore\. Each clay robot costs ([0-9]+) ore\. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay\. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian\.").unwrap();

        let datas = re
            .captures(raw)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut costs = HashMap::new();
        costs.insert(Material::Ore, vec![CostInfo::new(datas[1], Material::Ore)]);
        costs.insert(Material::Clay, vec![CostInfo::new(datas[2], Material::Ore)]);
        costs.insert(
            Material::Obsidian,
            vec![
                CostInfo::new(datas[3], Material::Ore),
                CostInfo::new(datas[4], Material::Clay),
            ],
        );
        costs.insert(
            Material::Geode,
            vec![
                CostInfo::new(datas[5], Material::Ore),
                CostInfo::new(datas[6], Material::Obsidian),
            ],
        );

        let mut robots = HashMap::new();
        robots.insert(Material::Ore, 0);
        robots.insert(Material::Clay, 0);
        robots.insert(Material::Obsidian, 0);
        robots.insert(Material::Geode, 0);

        let mut treasury = HashMap::new();
        treasury.insert(Material::Ore, 0);
        treasury.insert(Material::Clay, 0);
        treasury.insert(Material::Obsidian, 0);
        treasury.insert(Material::Geode, 0);

        Self {
            id: datas[0],
            costs,
            robots,
            treasury,
        }
    }

    // pub fn reset

    pub fn find_largest_number_of_geode(&mut self) -> usize {
        let mut elapsed_minutes: usize = 1;
        *self.robots.get_mut(&Material::Ore).unwrap() = 1;

        while elapsed_minutes < 24 {
            let mut bought = self.try_buying_robot(Material::Geode);
            if !bought {
                bought = self.try_buying_robot(Material::Obsidian);
            }
            if !bought && self.robots[&Material::Clay] < self.costs[&Material::Clay][0].value {
                bought = self.try_buying_robot(Material::Clay);
            }
            if !bought && self.robots[&Material::Ore] < self.costs[&Material::Ore][0].value {
                self.try_buying_robot(Material::Ore);
            }

            self.reward();
            elapsed_minutes += 1;
        }

        // println!("{}", self.robots[&Material::Obsidian]);
        *self.treasury.get(&Material::Geode).unwrap()
    }

    fn try_buying_robot(&mut self, speciality: Material) -> bool {
        let costs = &self.costs[&speciality];
        // checking
        for CostInfo { value, material } in costs {
            if self.treasury[material] < *value {
                return false;
            }
        }
        // buying
        for CostInfo { value, material } in costs {
            *self.treasury.get_mut(material).unwrap() -= value;
        }
        *self.robots.get_mut(&speciality).unwrap() += 1;
        true
    }

    fn reward(&mut self) {
        *self.treasury.get_mut(&Material::Ore).unwrap() += self.robots[&Material::Ore];
        *self.treasury.get_mut(&Material::Clay).unwrap() += self.robots[&Material::Clay];
        *self.treasury.get_mut(&Material::Obsidian).unwrap() += self.robots[&Material::Obsidian];
        *self.treasury.get_mut(&Material::Geode).unwrap() += self.robots[&Material::Geode];
    }
}
