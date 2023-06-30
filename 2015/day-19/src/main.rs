use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    // get input
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let mut input_parts = input.split("\n\n");

    let raw_replacements = input_parts.next().unwrap();
    let calibration_molecule = input_parts.next().unwrap();

    // parse input
    let mut replacements = HashMap::new();
    let mut reversed_replacements = HashMap::new();
    for replacement in raw_replacements.lines() {
        let mut datas = replacement.split(" => ");

        let (from, to) = (datas.next().unwrap(), datas.next().unwrap());
        replacements
            .entry(from)
            .and_modify(|counter: &mut Vec<&str>| counter.push(to))
            .or_insert(vec![to]);

        reversed_replacements.insert(to, from);
    }

    // p1
    let p1 = get_distinc_molecules(calibration_molecule, &replacements).len();
    println!("{p1}");

    // p2

    // this is not my implementation (see bellow), I gave up and looked for a solution online, https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4cu5b/
    // it's ridiculous to see that this works very efficiently. I'd never have thought that this'd works, and I don't fully understand why either -_-
    // it basically replaces everything it can until it cannot anymore or that it founds the solution. If it cannot replace anything anymore, it'll do it all over again but by randomizing the order in which it'll replace
    // like what the heck?! why is this working?? Without mentionning that it's super dupra supra fast 🙃
    let mut reps = reversed_replacements.into_iter().collect::<Vec<_>>();
    let mut target = calibration_molecule.to_string();
    let mut p2 = 0;
    while target != *"e" {
        let tmp = target.clone();
        for (a, b) in &reps {
            if !target.contains(a) {
                continue;
            }

            target = target.replacen(a, b, 1);
            p2 += 1;
        }

        if tmp == target {
            target = calibration_molecule.to_string();
            p2 = 0;
            reps.shuffle(&mut thread_rng()); // WHYYYY???
        }
    }
    println!("{p2}");

    // My original idea. It works for little inputs, but take way too long for the real input.
    // It works by walking backward (from the medical molecule to "e"); it's basically BFS, at each step it find all the possible backward replacements molecule and add it to the queue
    // but there is too much possibilities and even by sorting, we'd see the heat death of the universe before seeing the solution...
    /*
           let mut queue = vec![(calibration_molecule.to_string(), 0_usize)];
           let mut visited = vec![];

           let p2 = 'outer: loop {
               let (molecule, step) = queue.remove(0);
               let mut neighbours = get_distinc_reversed_molecules(&molecule, &reversed_replacements)
                   .iter()
                   .cloned()
                   .filter(|n| !visited.contains(n))
                   .collect::<Vec<_>>();
               neighbours.sort_by_key(|a| a.len());

               if neighbours.contains(&"e".to_string()) {
                   break 'outer step + 1;
               }

               visited.append(&mut neighbours.clone());
               queue.append(&mut neighbours.iter().map(|n| (n.clone(), step + 1)).collect());
           };

           fn get_distinc_reversed_molecules(
        calibration_molecule: &str,
        reversed_replacements: &HashMap<&str, &str>,
    ) -> Vec<String> {
        let mut molecules = vec![];
        let mol_chars = calibration_molecule.chars().collect::<Vec<_>>();

        for i in 0..mol_chars.len() {
            let fch = mol_chars[i];
            if i + 1 < mol_chars.len() {
                let mut mol = format!("{fch}");
                if reversed_replacements.contains_key(mol.as_str()) {
                    molecules.push((mol.clone(), i, i));
                }
                for (j, ch) in mol_chars.iter().enumerate().skip(i + 1) {
                    mol.push(*ch);
                    if reversed_replacements.contains_key(mol.as_str()) {
                        molecules.push((mol.clone(), i, j));
                    }
                }
            } else if reversed_replacements.contains_key(format!("{fch}").as_str()) {
                molecules.push((format!("{fch}"), i, i));
            }
        }

        let mut distinc_molecules: HashSet<String> = HashSet::new();
        for (molecule, start, end) in molecules {
            let replacement = &reversed_replacements[molecule.as_str()];
            let new_mol = format!(
                "{}{}{}",
                &calibration_molecule[..start],
                replacement,
                &calibration_molecule[end + 1..]
            );
            distinc_molecules.insert(new_mol);
        }
        distinc_molecules.iter().cloned().collect()
    }

        */
}

fn get_distinc_molecules(
    calibration_molecule: &str,
    replacements: &HashMap<&str, Vec<&str>>,
) -> Vec<String> {
    let mut molecules = vec![];
    let mol_chars = calibration_molecule.chars().collect::<Vec<_>>();
    for i in 0..mol_chars.len() {
        let (first_ch, second_ch) = (mol_chars[i], mol_chars.get(i + 1));
        if replacements.contains_key(format!("{first_ch}").as_str()) {
            molecules.push((format!("{first_ch}"), i, i));
        } else if second_ch.is_some()
            && replacements.contains_key(format!("{first_ch}{}", second_ch.unwrap()).as_str())
        {
            molecules.push((format!("{first_ch}{}", second_ch.unwrap()), i, i + 1));
        }
    }

    let mut distinc_molecules: HashSet<String> = HashSet::new();
    for (molecule, start, end) in molecules {
        let possible_replacements = &replacements[molecule.as_str()];
        for replacement in possible_replacements {
            let new_mol = format!(
                "{}{}{}",
                &calibration_molecule[..start],
                replacement,
                &calibration_molecule[end + 1..]
            );
            distinc_molecules.insert(new_mol);
        }
    }
    distinc_molecules.iter().cloned().collect()
}
