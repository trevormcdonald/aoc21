use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let start_fishies: Vec<i32> = contents
        .lines()
        .last()
        .unwrap()
        .split(',')
        .map(|f| f.parse().unwrap())
        .collect();

    // Part One
    let all_fishies = (0..80)
        .into_iter()
        .fold(start_fishies.clone(), |acc, _curr| {
            let mut new_fishies: Vec<i32> = vec![];
            acc.into_iter().for_each(|f| {
                if f == 0 {
                    new_fishies.push(6);
                    new_fishies.push(8);
                } else {
                    new_fishies.push(f - 1);
                }
            });
            new_fishies
        });

    println!("One: {:?}", all_fishies.len());

    // Part Two
    let mut fish_counters = vec![0_i64; 9];
    for f in start_fishies {
        fish_counters[f as usize] += 1;
    }

    (0..256).into_iter().for_each(|_day| {
        let num_new_fishies = fish_counters[0];
        fish_counters.rotate_left(1);
        fish_counters[6] += num_new_fishies
    });
    println!("Two: {:?}", fish_counters.iter().sum::<i64>());
}
