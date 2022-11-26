use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ProgramState {
    Running,
    WaitingForInput,
    Exited,
}

use ProgramState::*;

pub struct Program {
    memory: HashMap<usize, isize>,
    index: usize,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
    state: ProgramState,
    relative_base: isize,
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

    fn _get_args(&self) -> (isize, Vec<usize>, usize) {
        let (op, param_modes) = Program::_parse_op_code(self.read(self.index));
        let num_args = match op {
            1 => 3,  // add
            2 => 3,  // mul
            3 => 1,  // in
            4 => 1,  // out
            5 => 2,  // jump if true
            6 => 2,  // jump if false
            7 => 3,  // less than
            8 => 3,  // equals
            9 => 1,  // adjust relative base
            99 => 0, // exit
            _ => panic!("Invalid op code {}", op),
        };
        let args = (0..num_args)
            .map(|arg_index| {
                let param_mode = *param_modes.get(arg_index).unwrap_or(&0);
                match param_mode {
                    0 => self.read(self.index + arg_index + 1) as usize,
                    1 => self.index + arg_index + 1,
                    2 => (self.relative_base + self.read(self.index + arg_index + 1)) as usize,
                    _ => panic!("Invalid parameter mode {}", param_mode),
                }
            })
            .collect();
        (op, args, 1 + num_args)
    }

    pub fn new(intcode: &[isize]) -> Self {
        Self {
            memory: intcode.iter().enumerate().map(|(i, v)| (i, *v)).collect(),
            index: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            state: Running,
            relative_base: 0,
        }
    }

    pub fn read(&self, address: usize) -> isize {
        *self.memory.get(&address).unwrap_or(&0)
    }

    pub fn write(&mut self, address: usize, value: isize) {
        self.memory.insert(address, value);
    }

    pub fn send(&mut self, value: isize) {
        self.inputs.push_front(value);
    }

    pub fn receive(&mut self) -> Option<isize> {
        self.outputs.pop_back()
    }

    pub fn num_outputs(&self) -> usize {
        self.outputs.len()
    }

    pub fn state(&self) -> ProgramState {
        self.state
    }

    pub fn step(&mut self) {
        if self.state == Exited {
            return;
        }
        let mut increment_index = true;
        let (op, args, offset) = self._get_args();
        match op {
            1 => {
                // add
                let result = self.read(args[0]) + self.read(args[1]);
                // println!("&{} = {} + {}", args[2], self.read(args[0]), self.read(args[1]));
                self.write(args[2], result);
            }
            2 => {
                // mul
                let result = self.read(args[0]) * self.read(args[1]);
                // println!("&{} = {} * {}", args[2], self.read(args[0]), self.read(args[1]));
                self.write(args[2], result);
            }
            3 => {
                // in
                if let Some(input) = self.inputs.pop_back() {
                    // println!("&{} = {} (in)", args[0], input);
                    self.write(args[0], input);
                } else {
                    self.state = WaitingForInput;
                    return;
                }
            }
            4 => {
                // out
                // println!("{} (out)", self.read(args[0]));
                self.outputs.push_front(self.read(args[0]));
            }
            5 => {
                // jump if true
                // println!("if {} != 0 goto &{}", self.read(args[0]), self.read(args[1]));
                if self.read(args[0]) != 0 {
                    self.index = self.read(args[1]) as usize;
                    increment_index = false;
                }
            }
            6 => {
                // jump if false
                // println!("if {} == 0 goto &{}", self.read(args[0]), self.read(args[1]));
                if self.read(args[0]) == 0 {
                    self.index = self.read(args[1]) as usize;
                    increment_index = false;
                }
            }
            7 => {
                // less than
                let result = (self.read(args[0]) < self.read(args[1])) as isize;
                // println!("&{} = {} < {} ? 1 : 0", args[2], self.read(args[0]), self.read(args[1]));
                self.write(args[2], result);
            }
            8 => {
                // equals
                let result = (self.read(args[0]) == self.read(args[1])) as isize;
                // println!("&{} = {} == {} ? 1 : 0", args[2], self.read(args[0]), self.read(args[1]));
                self.write(args[2], result);
            }
            9 => {
                // adjust relative base
                // println!("rbase += {}", self.read(args[0]));
                self.relative_base += self.read(args[0]);
            }
            99 => {
                // exit
                // println!("EXIT");
                self.state = Exited;
                return;
            }
            _ => unreachable!(),
        }
        if increment_index {
            self.index += offset;
        }
        self.state = Running;
    }

    pub fn run_until<F>(programs: &mut [Program], pipes: &HashMap<usize, Vec<usize>>, condition: F)
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
