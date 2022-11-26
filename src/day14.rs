use itertools::Itertools;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

type Formulas = HashMap<String, (usize, Vec<(usize, String)>)>;

fn level(formulas: &Formulas, name: &str) -> usize {
    if name == "ORE" {
        return 0;
    }
    let (_, inputs) = formulas.get(name).unwrap();
    inputs
        .iter()
        .map(|(_, name)| level(formulas, name))
        .max()
        .unwrap()
        + 1
}

fn required_ores(formulas: &Formulas, required_fuel: usize) -> usize {
    let sorted_chemicals: Vec<_> = formulas
        .keys()
        .map(|name| (level(formulas, name), name))
        .sorted()
        .map(|(_, name)| name)
        .rev()
        .collect();
    let mut required_chemicals: HashMap<String, usize> = HashMap::new();
    required_chemicals.insert("FUEL".to_string(), required_fuel);
    for chemical in sorted_chemicals {
        let required_quantity = required_chemicals.get(chemical).unwrap();
        let (n_produced, inputs) = formulas.get(chemical).unwrap();
        let n_repeats = (required_quantity - 1) / n_produced + 1;
        required_chemicals.insert(chemical.clone(), n_repeats * n_produced);
        for (q, n) in inputs {
            let previous_qty = *required_chemicals.get(n).unwrap_or(&0);
            required_chemicals.insert(n.clone(), previous_qty + q * n_repeats);
        }
    }
    *required_chemicals.get("ORE").unwrap()
}

fn maximum_fuel(formulas: &Formulas, available_ores: usize) -> usize {
    let mut min_fuel = 1;
    let mut max_fuel = available_ores;
    while min_fuel != max_fuel {
        let fuel = (max_fuel + min_fuel) / 2;
        let ores = required_ores(formulas, fuel);
        match available_ores.cmp(&ores) {
            Ordering::Equal => {
                return fuel - 1;
            }
            Ordering::Less => {
                max_fuel = fuel - 1;
            }
            Ordering::Greater => {
                min_fuel = fuel;
            }
        }
    }
    min_fuel
}

pub fn run(input: &str) {
    let regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    let mut formulas: Formulas = HashMap::new();
    input.lines().for_each(|line| {
        let mut pairs: Vec<_> = regex
            .captures_iter(line)
            .map(|cap| (cap[1].parse().unwrap(), cap[2].to_string()))
            .collect();
        let (quantity, name) = pairs.pop().unwrap();
        formulas.insert(name, (quantity, pairs));
    });
    let num_ores = required_ores(&formulas, 1);
    println!("{}", num_ores);
    let max_fuel = maximum_fuel(&formulas, 1_000_000_000_000);
    println!("{}", max_fuel);
}
