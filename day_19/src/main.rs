use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut iter = input.split("\r\n\r\n");
    let replacements_s = iter.next().unwrap();
    let replacements = build_replacement_vec(replacements_s);
    let molecule = iter.next().unwrap().trim_end();

    let created_molecules_from_medicine = create_molecules(molecule, &replacements);

    println!("Part One: {}", created_molecules_from_medicine.len());
    println!("Part Two: {}", step_count_to_medicine(molecule));
}

fn build_replacement_vec(replacements_s: &str) -> Vec<(&str, &str)> {
    let mut replacements = Vec::new();
    for line in replacements_s.lines() {
        let mut line_split = line.split(" => ");
        let replace_from = line_split.next().unwrap();
        let replace_to = line_split.next().unwrap();
        replacements.push((replace_from, replace_to));
    }
    replacements
}

fn create_molecules(molecule: &str, replacements: &[(&str, &str)]) -> HashSet<String> {
    let mut created_molecules = HashSet::new();
    for (replace_from, replace_to) in replacements {
        let match_indices: Vec<_> = molecule.match_indices(replace_from).collect();
        for (index, _) in match_indices {
            let mut s = molecule.to_string();
            s.replace_range(index..index + replace_from.len(), replace_to);
            created_molecules.insert(s);
        }
    }
    created_molecules
}

fn build_molecule_vec(molecule: &str) -> Vec<String> {
    let molecule_chars: Vec<_> = molecule.chars().collect();
    let mut molecule_vec = Vec::new();
    let mut last_c_opt: Option<char> = None;

    for c in molecule_chars {
        if c.is_uppercase() {
            if let Some(last_c) = last_c_opt {
                molecule_vec.push(last_c.to_string());
            }
            last_c_opt = Some(c)
        } else if let Some(last_c) = last_c_opt {
            let mut s = last_c.to_string();
            s.push(c);
            molecule_vec.push(s);
            last_c_opt = None;
        }
    }
    if let Some(last_c) = last_c_opt {
        molecule_vec.push(last_c.to_string());
    }

    molecule_vec
}

fn step_count_to_medicine(molecule: &str) -> i32 {
    let molecule_v = build_molecule_vec(molecule);
    let mut steps = molecule_v.len() as i32 - 1;

    for molecule in molecule_v {
        steps -= match molecule.as_str() {
            "Rn" | "Ar" => 1,
            "Y" => 2,
            _ => 0,
        }
    }
    steps
}
