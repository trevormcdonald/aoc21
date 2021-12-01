use std::fs;

fn get_increases(acc: (i32, i32), x: &i32) -> (i32, i32) {
    if x > &acc.0 {
        (x.to_owned(), acc.1 + 1)
    } else {
        (x.to_owned(), acc.1)
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let ints: Vec<i32> = contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Part Uno
    let answer = ints.iter().fold((i32::MAX, 0), get_increases);
    println!("One: {:?}", answer.1);

    // Part Deux
    let windows: Vec<i32> = ints.windows(3).map(|w| w.iter().sum::<i32>()).collect();
    let answer_two = windows.iter().fold((i32::MAX, 0), get_increases);
    println!("Two: {:?}", answer_two.1);
}
