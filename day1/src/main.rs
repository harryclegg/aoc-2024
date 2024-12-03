use std::collections::HashMap;

fn main() {
    let file = include_str!("../input/main.txt");
    let lines = file.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i32>().unwrap());
        right.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    println!(
        "part 1: {}",
        left.iter().zip(&right).map(|(a, b)| (a - b).abs()).sum::<i32>()
    );

    let mut map: HashMap<i32, usize> = HashMap::new();
    for x in right {
        *map.entry(x).or_default() += 1;
    }

    let mut prod: i32 = 0;
    for a in left {
        let occurences : usize = *map.get(&a).unwrap_or(&0);
        prod += &a * occurences as i32;
    }

    println!(
        "part 2: {}", prod);
}
