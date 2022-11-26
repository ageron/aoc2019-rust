fn pattern(for_output: usize, index: usize) -> isize {
    let index = (index + 1) / (for_output + 1);
    [0, 1, 0, -1][index % 4]
}

fn signal_to_str(signal: &[u8]) -> String {
    let bytes = signal.iter().map(|c| *c + b'0').collect();
    String::from_utf8(bytes).unwrap()
}

fn phase_part1(signal: &[u8]) -> Vec<u8> {
    let mut new_signal: Vec<u8> = Vec::with_capacity(signal.len());
    for for_output in 0..signal.len() {
        new_signal.push(
            (signal
                .iter()
                .enumerate()
                .map(|(index, val)| pattern(for_output, index) * (*val as isize))
                .sum::<isize>()
                .unsigned_abs()
                % 10) as u8,
        );
    }
    new_signal
}

fn get_message_part1(signal: &[u8]) -> String {
    let mut new_signal = signal.to_vec();
    for _ in 0..100 {
        new_signal = phase_part1(&new_signal);
    }
    signal_to_str(&new_signal[0..8])
}

fn get_message_part2(signal: &[u8]) -> String {
    let start_index = signal
        .iter()
        .take(7)
        .fold(0, |acc, c| acc * 10 + *c as usize);
    let total_length = signal.len() * 10_000;
    assert!(start_index >= total_length / 2); // only works when looking for digits in the second half
    let remaining_length = total_length - start_index;
    let mut new_signal = Vec::with_capacity(remaining_length);
    for index in 0..remaining_length {
        new_signal.push(signal[(start_index + index) % signal.len()]);
    }
    for _ in 0..100 {
        let mut total: u8 = 0;
        for index in (0..(remaining_length - 1)).rev() {
            total = (total + new_signal[index]) % 10;
            new_signal[index] = total;
        }
    }
    signal_to_str(&new_signal[0..8])
}

pub fn run(input: &str) {
    let signal: Vec<u8> = input.bytes().map(|b| b - b'0').collect();
    let message = get_message_part1(&signal);
    println!("{}", message);
    let message = get_message_part2(&signal);
    println!("{}", message);
}
