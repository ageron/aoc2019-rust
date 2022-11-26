fn run_intcode(intcode: &[isize], code1: isize, code2: isize) -> isize {
    let mut intcode = intcode.to_vec();
    intcode[1] = code1;
    intcode[2] = code2;
    for i in (0..intcode.len()).step_by(4) {
        let op = intcode[i];
        let arg0 = intcode[intcode[i + 1] as usize];
        let arg1 = intcode[intcode[i + 2] as usize];
        let result_index = intcode[i + 3] as usize;
        // println!("{op} {arg0} {arg1} {result_index}");
        match op {
            99 => break,
            1 => {
                intcode[result_index] = arg0 + arg1;
            }
            2 => {
                intcode[result_index] = arg0 * arg1;
            }
            _ => unreachable!(),
        }
    }
    intcode[0]
}

fn search_noun_verb(intcode: &[isize], target: isize) -> isize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run_intcode(intcode, noun, verb);
            if result == target {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Noun-verb not found!")
}

pub fn run(input: &str) {
    let intcode: Vec<isize> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let code0 = run_intcode(&intcode, 12, 2);
    println!("{}", code0);
    let noun_verb = search_noun_verb(&intcode, 19690720);
    println!("{}", noun_verb);
}
