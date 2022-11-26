fn get_layers(nums: &[u8], width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
    let layer_size = width * height;
    let num_layers = nums.len() / layer_size;
    assert!(width * height * num_layers == nums.len());
    (0..num_layers)
        .map(|layer_index| {
            (0..height)
                .map(|row_index| {
                    let index = layer_size * layer_index + row_index * width;
                    nums[index..index + width].to_vec()
                })
                .collect()
        })
        .collect()
}

fn count_digit(layer: &[Vec<u8>], digit: u8) -> usize {
    layer
        .iter()
        .map(|row| {
            row.iter().filter(|c| **c == digit).count() // number of zeros per row
        })
        .sum() // number of zeros per layer
}

fn corruption_check(layers: &[Vec<Vec<u8>>]) -> usize {
    let count_zeros: Vec<_> = layers.iter().map(|layer| count_digit(layer, 0)).collect();
    let min_zeros = count_zeros.iter().min().unwrap();
    let min_zero_index = count_zeros
        .iter()
        .position(|count| count == min_zeros)
        .unwrap();
    let layer_with_min_zeros = &layers[min_zero_index];
    count_digit(layer_with_min_zeros, 1) * count_digit(layer_with_min_zeros, 2)
}

fn is_white_pixel(layers: &Vec<Vec<Vec<u8>>>, x: usize, y: usize) -> bool {
    for layer in layers {
        let pixel = layer[y][x];
        match pixel {
            0 => return false,
            1 => return true,
            2 => {}
            _ => unreachable!(),
        }
    }
    panic!("Unknown background color")
}

fn display_image(layers: &Vec<Vec<Vec<u8>>>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let is_white = is_white_pixel(layers, x, y);
            print!("{}", if is_white { "█" } else { " " });
        }
        println!();
    }
}

pub fn run(input: &str) {
    let nums: Vec<u8> = input.bytes().map(|c| c - b'0').collect();
    let width = 25;
    let height = 6;
    let layers = get_layers(&nums, width, height);
    let result = corruption_check(&layers);
    println!("{}", result);
    display_image(&layers, width, height);
}
