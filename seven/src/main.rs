use std::fs;

fn calculate_fuel_cost_one(positions: &Vec<usize>, align: &usize) -> usize {
    let mut cost = 0;
    for p in positions {
        cost += if p > align { p - align } else { align - p };
    }
    cost
}

fn calculate_fuel_cost_two(positions: &Vec<usize>, align: &usize) -> usize {
    let mut cost = 0;
    for p in positions {
        cost += if p > align {
            (1..=(p - align)).into_iter().sum::<usize>()
        } else {
            (1..=(align - p)).into_iter().sum::<usize>()
        };
    }
    cost
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let initial_positions: Vec<usize> = contents
        .lines()
        .last()
        .unwrap()
        .split(',')
        .map(|f| f.parse().unwrap())
        .collect();
    let min = initial_positions.clone().into_iter().min().unwrap();
    let max = initial_positions.clone().into_iter().max().unwrap();

    let fuel_one = (min..max)
        .into_iter()
        .map(|x| calculate_fuel_cost_one(&initial_positions, &x))
        .min()
        .unwrap();
    println!("One: {:?}", fuel_one);

    let fuel_two = (min..max)
        .into_iter()
        .map(|x| calculate_fuel_cost_two(&initial_positions, &x))
        .min()
        .unwrap();
    println!("Two: {:?}", fuel_two);
}
