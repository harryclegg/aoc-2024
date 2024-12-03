fn process(input: &Vec<i32>) -> Vec<bool> {
    let diff = &input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    diff.iter()
        .map(|&x| x.abs() >= 1 && x.abs() <= 3 && (x > 0) == (diff[0] > 0))
        .collect::<Vec<_>>()
}

fn check(input: &Vec<i32>) -> bool {
    let in_range = process(&input);

    in_range
        .iter()
        .filter(|x| **x == true)
        .collect::<Vec<_>>()
        .len()
        == input.len() - 1
}

fn test(input: &Vec<i32>, remove: bool) -> bool {
    if check(&input) {
        return true;
    } else if remove {
        for elem in 0..input.len() {
            let mut vec_copy = input.clone();
            vec_copy.remove(elem);
            let result = check(&vec_copy);
            if result {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let file = include_str!("../input/input.txt");
    let lines = file.lines();

    let mut count_1: i32 = 0;
    let mut count_2: i32 = 0;

    for line in lines {
        let vector: Vec<_> = line
            .split_whitespace()
            .map(|x| str::parse::<i32>(x).ok().unwrap())
            .collect();

        count_1 += test(&vector, false) as i32;
        count_2 += test(&vector, true) as i32;
    }
    println!("part 1: {}", count_1);
    println!("part 2: {}", count_2);
}
