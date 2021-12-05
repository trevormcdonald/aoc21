use std::fs;

const FIELD_DIM: usize = 1000;

type Field = Vec<Vec<i32>>;

type Line = (Vec<usize>, Vec<usize>);

fn str_to_line(s: &str) -> Line {
    let mut str_iter = s.split_whitespace();
    let v1 = str_iter.next().unwrap();
    let v2 = str_iter.last().unwrap();
    (
        v1.split(',').map(|x| x.parse().unwrap()).collect(),
        v2.split(',').map(|x| x.parse().unwrap()).collect(),
    )
}

fn cover_one(field: &mut Field, line: Line) {
    let x1 = line.0.get(0).unwrap().to_owned();
    let y1 = line.0.get(1).unwrap().to_owned();
    let x2 = line.1.get(0).unwrap().to_owned();
    let y2 = line.1.get(1).unwrap().to_owned();

    if x1 == x2 {
        (y1..=y2).into_iter().for_each(|y| {
            field[x1][y] += 1;
        });
        (y2..=y1).into_iter().for_each(|y| {
            field[x1][y] += 1;
        });
    } else if y1 == y2 {
        (x1..=x2).into_iter().for_each(|x| {
            field[x][y1] += 1;
        });
        (x2..=x1).into_iter().for_each(|x| {
            field[x][y1] += 1;
        })
    }
}

fn cover_two(field: &mut Field, line: Line) {
    let x1 = line.0.get(0).unwrap().to_owned();
    let y1 = line.0.get(1).unwrap().to_owned();
    let x2 = line.1.get(0).unwrap().to_owned();
    let y2 = line.1.get(1).unwrap().to_owned();

    if x1 == x2 {
        (y1..=y2).into_iter().for_each(|y| {
            field[x1][y] += 1;
        });
        (y2..=y1).into_iter().for_each(|y| {
            field[x1][y] += 1;
        });
    } else if y1 == y2 {
        (x1..=x2).into_iter().for_each(|x| {
            field[x][y1] += 1;
        });
        (x2..=x1).into_iter().for_each(|x| {
            field[x][y1] += 1;
        })
    } else {
        let xs: Vec<usize> = if !(x1..=x2).is_empty() {
            (x1..=x2).into_iter().collect()
        } else {
            (x2..=x1).rev().into_iter().collect()
        };
        let ys: Vec<usize> = if !(y1..=y2).is_empty() {
            (y1..=y2).into_iter().collect()
        } else {
            (y2..=y1).rev().into_iter().collect()
        };
        xs.into_iter().zip(ys.into_iter()).for_each(|(x, y)| {
            field[x][y] += 1;
        })
    }
}

fn tally(field: Field) -> usize {
    field.into_iter().fold(0, |acc, curr| {
        let row_count = curr.into_iter().filter(|i| i >= &2).count();
        acc + row_count
    })
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<Line> = contents.lines().map(str_to_line).collect();

    // Part One
    let mut field_one = vec![vec![0; FIELD_DIM]; FIELD_DIM];
    lines
        .clone()
        .into_iter()
        .for_each(|line| cover_one(&mut field_one, line));

    println!("One: {:?}", tally(field_one));

    // Part Two
    let mut field_two = vec![vec![0; FIELD_DIM]; FIELD_DIM];
    lines
        .into_iter()
        .for_each(|line| cover_two(&mut field_two, line));

    println!("Two: {:?}", tally(field_two));
}
