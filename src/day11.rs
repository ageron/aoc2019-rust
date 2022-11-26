use itertools::Itertools;
use std::collections::HashMap;

use super::intcode::{Program, ProgramState::*};

fn paint_spaceship(intcode: &[isize], display: bool) -> usize {
    let mut panels: HashMap<(isize, isize), bool> = HashMap::new();
    let mut brain = Program::new(intcode);
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;
    if display {
        panels.insert((0, 0), true);
    }
    loop {
        let is_white = *panels.get(&(x, y)).unwrap_or(&false);
        brain.send(is_white as isize);
        while brain.state() == Running && brain.num_outputs() < 2 {
            brain.step();
        }
        if brain.num_outputs() >= 2 {
            let is_paint_white = brain.receive().unwrap() == 1;
            panels.insert((x, y), is_paint_white);
            let is_turn_right = brain.receive().unwrap() == 1;
            if is_turn_right {
                (dx, dy) = (-dy, dx);
            } else {
                (dx, dy) = (dy, -dx);
            }
            x += dx;
            y += dy;
        }
        if brain.state() == Exited {
            break;
        }
    }
    if display {
        let (min_x, max_x) = panels
            .keys()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = panels
            .keys()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                let is_white = *panels.get(&(x, y)).unwrap_or(&false);
                print!("{}", if is_white { '█' } else { ' ' });
            }
            println!();
        }
    }
    panels.len()
}

pub fn run(input: &str) {
    let intcode: Vec<isize> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let num_painted = paint_spaceship(&intcode, false);
    println!("{}", num_painted);
    paint_spaceship(&intcode, true);
}
