use itertools::Itertools;
use std::collections::HashMap;

fn count_orbits(parent_to_child: &HashMap<&str, Vec<&str>>, parent: &str, level: usize) -> usize {
    level
        + parent_to_child.get(parent).map_or(0, |children| {
            children
                .iter()
                .map(|child| count_orbits(parent_to_child, child, level + 1))
                .sum()
        })
}

fn min_orbital_transfers(
    child_to_parent: &HashMap<&str, &str>,
    object1: &str,
    object2: &str,
) -> usize {
    let mut steps: HashMap<&str, usize> = HashMap::from([(object1, 0), (object2, 0)]);
    let mut objects = [object1, object2];
    for step in 1.. {
        for object_index in [0, 1] {
            let child = objects[object_index];
            let parent = child_to_parent.get(child);
            if let Some(parent) = parent {
                if let Some(other_steps) = steps.get(parent) {
                    return other_steps + step;
                }
                steps.insert(parent, step);
                objects[object_index] = parent;
            }
        }
    }
    unreachable!()
}

pub fn run(input: &str) {
    let mut parent_to_child: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut child_to_parent: HashMap<&str, &str> = HashMap::new();
    input.lines().for_each(|line| {
        let (parent, child): (&str, &str) = line.split(')').collect_tuple().unwrap();
        let children = parent_to_child.entry(parent).or_insert(vec![]);
        children.push(child);
        child_to_parent.insert(child, parent);
    });
    let count = count_orbits(&parent_to_child, "COM", 0);
    println!("{}", count);
    let transfers = min_orbital_transfers(
        &child_to_parent,
        child_to_parent.get("YOU").unwrap(),
        child_to_parent.get("SAN").unwrap(),
    );
    println!("{}", transfers);
}
