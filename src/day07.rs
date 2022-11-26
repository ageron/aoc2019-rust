use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;

#[derive(Eq, PartialEq)]
enum ProgramState {
    Running,
    WaitingForInput,
    Exited,
}

use ProgramState::*;

struct Program {
    memory: Vec<isize>,
    index: usize,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
    state: ProgramState,
}

impl Program {
    fn _parse_op_code(op_code: isize) -> (isize, Vec<isize>) {
        let op = op_code % 100;
        let mut modes = op_code / 100;
        let mut param_modes: Vec<isize> = vec![];
        while modes > 0 {
            param_modes.push(modes % 10);
            modes /= 10;
        }
        (op, param_modes)
    }

    fn _get_args(&self) -> (isize, Vec<isize>, Option<usize>, usize) {
        let (op, param_modes) = Program::_parse_op_code(self.memory[self.index]);
        let (num_args, has_result) = match op {
            1 => (2, true),   // add
            2 => (2, true),   // mul
            3 => (0, true),   // in
            4 => (1, false),  // out
            5 => (2, false),  // jump if true
            6 => (2, false),  // jump if false
            7 => (2, true),   // less than
            8 => (2, true),   // equals
            99 => (0, false), // exit
            _ => panic!("Invalid op code {}", op),
        };
        let args = (0..num_args)
            .map(|arg_index| {
                let param_mode = *param_modes.get(arg_index).unwrap_or(&0);
                let val = self.memory[self.index + arg_index + 1];
                match param_mode {
                    0 => self.memory[val as usize],
                    1 => val,
                    _ => panic!("Invalid parameter mode {}", param_mode),
                }
            })
            .collect();
        let result_index = if has_result {
            Some(self.memory[self.index + num_args + 1] as usize)
        } else {
            None
        };
        (op, args, result_index, 1 + num_args + has_result as usize)
    }

    fn new(intcode: &[isize]) -> Self {
        Self {
            memory: intcode.to_vec(),
            index: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            state: Running,
        }
    }

    pub fn step(&mut self) {
        if self.state == Exited {
            return;
        }
        let mut increment_index = true;
        let (op, args, result_index, offset) = self._get_args();
        match op {
            1 => {
                // add
                self.memory[result_index.unwrap()] = args[0] + args[1];
            }
            2 => {
                // mul
                self.memory[result_index.unwrap()] = args[0] * args[1];
            }
            3 => {
                // in
                if let Some(input) = self.inputs.pop_back() {
                    self.memory[result_index.unwrap()] = input;
                } else {
                    self.state = WaitingForInput;
                    return;
                }
            }
            4 => {
                // out
                self.outputs.push_front(args[0]);
            }
            5 => {
                // jump if true
                if args[0] != 0 {
                    self.index = args[1] as usize;
                    increment_index = false;
                }
            }
            6 => {
                // jump if false
                if args[0] == 0 {
                    self.index = args[1] as usize;
                    increment_index = false;
                }
            }
            7 => {
                // less than
                self.memory[result_index.unwrap()] = (args[0] < args[1]) as isize;
            }
            8 => {
                // equals
                self.memory[result_index.unwrap()] = (args[0] == args[1]) as isize;
            }
            99 => {
                self.state = Exited;
                return;
            } // exit
            _ => unreachable!(),
        }
        if increment_index {
            self.index += offset;
        }
        self.state = Running;
    }

    fn run_until<F>(programs: &mut [Program], pipes: &HashMap<usize, Vec<usize>>, condition: F)
    where
        F: Fn(&[Program]) -> bool,
    {
        while !condition(programs) {
            for index in 0..programs.len() {
                programs[index].step();
                let send_to_indices = pipes.get(&index);
                if let Some(send_to_indices) = send_to_indices {
                    for send_to_index in send_to_indices {
                        programs[*send_to_index]
                            .inputs
                            .append(&mut programs[index].outputs.clone());
                    }
                    programs[index].outputs = VecDeque::new();
                }
            }
        }
    }
}

fn run_single_chain(intcode: &[isize], phases: &[isize], with_cycle: bool) -> isize {
    // init each program with the corresponding phase input
    let mut programs: Vec<_> = phases
        .iter()
        .map(|phase| {
            let mut program = Program::new(intcode);
            program.inputs.push_front(*phase);
            program
        })
        .collect();
    // connect each program to the next
    let mut pipes: HashMap<usize, Vec<usize>> = HashMap::new();
    (0..4).for_each(|i| {
        pipes.insert(i, vec![i + 1]);
    });
    if with_cycle {
        pipes.insert(4, vec![0]);
    }
    // send first signal to the first program
    programs[0].inputs.push_front(0);
    // run until program #4 ends
    Program::run_until(&mut programs, &pipes, |programs| {
        programs[4].state == Exited
    });
    // return program #4's last output signal
    if with_cycle {
        // the last output has already been piped to program #0's inputs
        *programs[0].inputs.front().unwrap()
    } else {
        *programs[4].outputs.front().unwrap()
    }
}

fn highest_signal(intcode: &[isize], phases_range: Range<isize>, with_cycle: bool) -> isize {
    phases_range
        .permutations(5)
        .map(|phases| run_single_chain(intcode, &phases, with_cycle))
        .max()
        .unwrap()
}

pub fn run(input: &str) {
    let intcode: Vec<isize> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let thruster_signal = highest_signal(&intcode, 0..5, false);
    println!("{}", thruster_signal);
    let thruster_signal = highest_signal(&intcode, 5..10, true);
    println!("{}", thruster_signal);
}
