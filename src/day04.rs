use itertools::Itertools;

fn is_valid_password(password: usize, with_solitary_pair: bool) -> bool {
    let mut val = password;
    let mut previous_digit = val % 10;
    let mut has_repetition = false;
    let mut repetitions = 0;
    while val != 0 {
        val /= 10;
        let digit = val % 10;
        if digit > previous_digit {
            return false;
        } // decreasing digits
        if digit == previous_digit {
            repetitions += 1;
            if !with_solitary_pair {
                has_repetition = true;
            }
        } else {
            if with_solitary_pair && repetitions == 1 {
                has_repetition = true;
            }
            repetitions = 0;
        }
        previous_digit = digit;
    }
    has_repetition
}

fn count_valid_passwords(min_value: usize, max_value: usize, with_solitary_pair: bool) -> usize {
    (min_value..=max_value)
        .filter(|p| is_valid_password(*p, with_solitary_pair))
        .count()
}

pub fn run(input: &str) {
    let (min_value, max_value): (usize, usize) = input
        .split('-')
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let count = count_valid_passwords(min_value, max_value, false);
    println!("{}", count);
    let count = count_valid_passwords(min_value, max_value, true);
    println!("{}", count);
}
