use std::collections::VecDeque;

use crate::intcode::{Program, ProgramState::*};

fn run_network(intcode: &[isize], num_computers: usize, with_nat: bool) -> isize {
    let mut programs: Vec<Program> = (0..num_computers)
        .map(|address| {
            let mut program = Program::new(intcode);
            program.send(address as isize);
            program
        })
        .collect();
    let mut queues: Vec<VecDeque<(isize, isize)>> =
        (0..num_computers).map(|_| VecDeque::new()).collect();
    let mut idle_count: Vec<usize> = vec![0; num_computers];
    let mut nat_memory: (isize, isize) = (-1, -1);
    let mut previous_y = -1;
    loop {
        for index in 0..num_computers {
            loop {
                programs[index].step();
                if programs[index].state() == WaitingForInput {
                    break;
                }
            }
            if queues[index].is_empty() {
                programs[index].send(-1);
                programs[index].step();
                idle_count[index] += 1;
            } else {
                let (x, y) = queues[index].pop_back().unwrap();
                for value in [x, y] {
                    programs[index].send(value);
                    loop {
                        programs[index].step();
                        if programs[index].state() != Running {
                            break;
                        }
                    }
                }
                idle_count[index] = 0;
            }
            while programs[index].num_outputs() >= 3 {
                let destination = programs[index].receive().unwrap();
                let x = programs[index].receive().unwrap();
                let y = programs[index].receive().unwrap();
                if destination == 255 {
                    if with_nat {
                        nat_memory = (x, y);
                    } else {
                        return y;
                    }
                } else {
                    queues[destination as usize].push_front((x, y));
                    idle_count[destination as usize] = 0;
                }
            }
        }
        let idle_threshold = 1;
        if with_nat
            && (idle_count.iter().all(|v| *v > idle_threshold))
            && (queues.iter().all(|queue| queue.is_empty()))
        {
            queues[0].push_front(nat_memory);
            idle_count[0] = 0;
            if nat_memory.1 == previous_y {
                return previous_y;
            }
            previous_y = nat_memory.1;
        }
    }
}

pub fn run(input: &str) {
    let intcode: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let num_computers = 50;
    let broadcast_y = run_network(&intcode, num_computers, false);
    println!("{}", broadcast_y);
    let repeated_y = run_network(&intcode, num_computers, true);
    println!("{}", repeated_y);
}
