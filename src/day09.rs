use hashbrown::HashMap;

// this version of the intcode computer turned out to be the final one, so I
// exported it to a separate module so I can reuse it later:
use super::intcode::{Program, ProgramState::*};

fn get_output(intcode: &[isize], input: isize) -> isize {
    let mut progs = [Program::new(intcode)];
    progs[0].send(input);
    Program::run_until(&mut progs, &HashMap::new(), |progs: &[Program]| {
        progs[0].state() == Exited
    });
    // dbg!(&progs[0].outputs);
    progs[0].receive().unwrap()
}

pub fn run(input: &str) {
    let intcode: Vec<isize> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let boost_keycode = get_output(&intcode, 1);
    println!("{}", boost_keycode);
    let distress_signal = get_output(&intcode, 2);
    println!("{}", distress_signal);
}
