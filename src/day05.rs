fn parse_op_code(op_code: isize) -> (isize, Vec<isize>) {
    let op = op_code % 100;
    let mut modes = op_code / 100;
    let mut param_modes: Vec<isize> = vec![];
    while modes > 0 {
        param_modes.push(modes % 10);
        modes /= 10;
    }
    (op, param_modes)
}

fn get_args(intcode: &[isize], index: usize) -> (isize, Vec<isize>, Option<usize>, usize) {
    let (op, param_modes) = parse_op_code(intcode[index]);
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
            let val = intcode[index + arg_index + 1];
            match param_mode {
                0 => intcode[val as usize],
                1 => val,
                _ => panic!("Invalid parameter mode {}", param_mode),
            }
        })
        .collect();
    let result_index = if has_result {
        Some(intcode[index + num_args + 1] as usize)
    } else {
        None
    };
    (op, args, result_index, 1 + num_args + has_result as usize)
}

fn run_intcode(intcode: &[isize], inputs: &[isize]) -> isize {
    let mut intcode = intcode.to_vec();
    let mut index = 0;
    let mut input_iter = inputs.iter();
    let mut outputs: Vec<isize> = vec![];
    loop {
        let mut increment_index = true;
        let (op, args, result_index, offset) = get_args(&intcode, index);
        match op {
            1 => {
                // add
                intcode[result_index.unwrap()] = args[0] + args[1];
            }
            2 => {
                // mul
                intcode[result_index.unwrap()] = args[0] * args[1];
            }
            3 => {
                // in
                intcode[result_index.unwrap()] = *input_iter.next().unwrap();
            }
            4 => {
                // out
                outputs.push(args[0]);
            }
            5 => {
                // jump if true
                if args[0] != 0 {
                    index = args[1] as usize;
                    increment_index = false;
                }
            }
            6 => {
                // jump if false
                if args[0] == 0 {
                    index = args[1] as usize;
                    increment_index = false;
                }
            }
            7 => {
                // less than
                intcode[result_index.unwrap()] = (args[0] < args[1]) as isize;
            }
            8 => {
                // equals
                intcode[result_index.unwrap()] = (args[0] == args[1]) as isize;
            }
            99 => break, // exit
            _ => unreachable!(),
        }
        if increment_index {
            index += offset;
        }
    }
    assert!(outputs[0..outputs.len() - 1].iter().all(|v| *v == 0));
    *outputs.iter().last().unwrap()
}

pub fn run(input: &str) {
    let intcode: Vec<isize> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let diagnostic_code = run_intcode(&intcode, &[1]);
    println!("{}", diagnostic_code);
    let diagnostic_code = run_intcode(&intcode, &[5]);
    println!("{}", diagnostic_code);
}
