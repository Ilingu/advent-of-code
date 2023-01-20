use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
pub struct Valve {
    name: String,
    neighbourg: Vec<String>,
    pressure: usize,
}

#[derive(Debug)]
pub struct Graph {
    graph: HashMap<String, Valve>,
    nodes_len: usize,
}

impl Graph {
    pub fn parse(input: &str) -> Self {
        let mut graph: HashMap<String, Valve> = HashMap::new();

        for line in input.lines() {
            let datas = line.split_whitespace().collect::<Vec<&str>>();
            assert!(datas.len() >= 9);

            let node_name = datas[1].to_string();
            let pressure = datas[4]
                .trim_end_matches(";")
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let neighbourg: Vec<String> = (&datas[9..=datas.len() - 1])
                .iter()
                .map(|x| x.trim_matches(',').to_string())
                .collect();

            graph.insert(
                node_name.clone(),
                Valve {
                    name: node_name,
                    pressure,
                    neighbourg,
                },
            );
        }

        return Self {
            graph,
            nodes_len: input.lines().count(),
        };
    }

    pub fn get_best_pressure_score(&self) -> usize {
        let mut minutes: usize = 0;

        let mut current_node = "AA".to_string();
        let mut ratio_history: HashMap<String, f32> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();

        let mut score: usize = 0;
        loop {
            let dists = self.shortest_path(&current_node);

            let mut path: Vec<(String, usize)> = vec![];
            for (n, d) in dists {
                let node = self.graph.get(&n).unwrap();
                if node.pressure == 0 || visited.contains(&n) {
                    continue;
                }

                let ratio = (node.pressure as f32 + minutes as f32) / (d as f32);
                let node_ratio = ratio_history.get_mut(&n);
                if node_ratio.is_some() {
                    *node_ratio.unwrap() *= ratio;
                } else {
                    ratio_history.insert(n.clone(), ratio);
                }

                path.push((n, d));
            }
            if path.len() <= 0 {
                return score;
            }

            path.sort_by(|a, b| {
                let (ratio_a, ratio_b): (f32, f32) = (
                    *ratio_history.get(&a.0).unwrap(),
                    *ratio_history.get(&b.0).unwrap(),
                );
                return ratio_b.total_cmp(&ratio_a);
            });
            println!("{:?}", path);

            let (next_node_name, next_node_dist) = path.first().unwrap();
            visited.insert(next_node_name.clone());

            minutes += next_node_dist + 1;
            if minutes > 30 {
                return score;
            }

            let next_node = self.graph.get(next_node_name).unwrap();
            score += (30 - minutes) * next_node.pressure;

            current_node = next_node_name.clone();
        }
    }

    pub fn shortest_path(&self, from: &String) -> HashMap<String, usize> {
        if !self.graph.contains_key(from) {
            panic!("node doesn't exist")
        }

        let mut queue: Vec<(String, usize)> = vec![];
        let mut distance: HashMap<String, usize> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();

        queue.push((from.clone(), 0));
        while queue.len() > 0 {
            queue.sort_by_key(|x| x.1);
            let (name, best_dist) = queue.first().unwrap().clone();
            visited.insert(name.clone());

            let node_dist = distance.get(&name);
            if node_dist.is_some() && &best_dist > node_dist.unwrap() {
                queue.remove(0);
                continue;
            }

            let node = self.graph.get(&name).unwrap();
            for neighbourg in &node.neighbourg {
                if visited.contains(neighbourg) {
                    continue;
                }

                let dist = best_dist + 1;
                queue.push((neighbourg.to_string(), dist));

                let neighdist = distance.get(neighbourg);
                if neighdist.is_some() && neighdist.unwrap() < &dist {
                    continue;
                }

                distance.remove(neighbourg);
                distance.insert(neighbourg.to_string(), dist);
            }

            queue.remove(0);
        }

        return distance;
    }
}
