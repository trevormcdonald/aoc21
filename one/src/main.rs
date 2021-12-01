use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let answer = lines.fold((i64::MAX, 0), |acc, x| {
        let parsed_value = x.parse::<i64>().unwrap();
        if parsed_value > acc.0 {
            (parsed_value, acc.1 + 1)
        } else {
            (parsed_value, acc.1)
        }
    });
    println!("{:?}", answer);

    // Part Deux
    let ints: Vec<i64> = contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let answer_two = ints
        .windows(3)
        .map(|w| w.iter().sum())
        .fold((i64::MAX, 0), |acc, x| {
            if x > acc.0 {
                (x, acc.1 + 1)
            } else {
                (x, acc.1)
            }
        });

    println!("{:?}", answer_two);
}
