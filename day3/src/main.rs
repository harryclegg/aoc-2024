use regex::Regex;

const INSTRUCTION_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

fn run(re: &Regex, input: &str) -> i32 {
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
    }

    let multiplied = results.iter().map(|x| x.0 * x.1).collect::<Vec<_>>();
    multiplied.iter().sum()
}

fn main() {
    let file = include_str!("../input/input.txt");

    let instruction_re = Regex::new(INSTRUCTION_REGEX).unwrap();

    println!("part 1: {}", run(&instruction_re, file));

    let data = file.split("do()");
    let mut sum: i32 = 0;
    for line in data {
        sum += run(&instruction_re, line.split("don't").next().unwrap());
    }

    println!("part 2: {}", sum);
}
