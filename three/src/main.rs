use std::fs;

fn get_num_ones_at_position(v: Vec<Vec<usize>>, position: usize) -> usize {
    v.iter().map(|x| x.get(position).unwrap()).sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let nums: Vec<Vec<usize>> = contents
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(2).unwrap() as usize).collect())
        .collect();

    let bit_length = nums.get(0).unwrap().len();

    // Part One
    let total = nums.len();

    let bit_modes = nums.iter().fold(vec![0; bit_length], |acc, x| {
        acc.iter().zip(x.iter()).map(|x| x.0 + x.1).collect()
    });

    let gamma: Vec<&str> = bit_modes
        .iter()
        .map(|x| if x > &(total / 2) { "1" } else { "0" })
        .collect();

    let epsilon: Vec<&str> = bit_modes
        .iter()
        .map(|x| if x < &(total / 2) { "1" } else { "0" })
        .collect();

    println!(
        "One: {:?}",
        usize::from_str_radix(&gamma.concat(), 2).unwrap()
            * usize::from_str_radix(&epsilon.concat(), 2).unwrap()
    );

    // Part Two
    let oxygen = (0..bit_length)
        .into_iter()
        .fold(nums.clone(), |acc, curr_position| {
            acc.iter()
                .filter(|x| {
                    if acc.len() > 1 {
                        (get_num_ones_at_position(acc.clone(), curr_position) * 2 >= acc.len()) // ties go to one
                            == (x.get(curr_position).unwrap() == &1_usize)
                    } else {
                        true
                    }
                })
                .map(|x| x.to_owned())
                .collect()
        })
        .get(0)
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat();

    let co2 = (0..bit_length)
        .into_iter()
        .fold(nums, |acc, curr_position| {
            acc.iter()
                .filter(|x| {
                    if acc.len() > 1 {
                        (get_num_ones_at_position(acc.clone(), curr_position) * 2 < acc.len()) //ties go to zero
                            == (x.get(curr_position).unwrap() == &1_usize)
                    } else {
                        true
                    }
                })
                .map(|x| x.to_owned())
                .collect()
        })
        .get(0)
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat();

    println!(
        "Two: {:?}",
        usize::from_str_radix(&oxygen, 2).unwrap() * usize::from_str_radix(&co2, 2).unwrap()
    );
}
