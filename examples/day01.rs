use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day01.txt").unwrap();

    let parsed = input
        .split("\n")
        .map(|raw_line| {
            let temp: Vec<char> = raw_line.chars().filter(|c| c.is_digit(10)).collect();

            if temp.len() > 1 {
                let first = temp[0];
                let last = temp[temp.len() - 1];

                return format!("{first}{last}").parse::<u32>().unwrap();
            } else {
                return format!("{}{}", temp[0], temp[0]).parse::<u32>().unwrap();
            }
        })
        .collect::<Vec<u32>>();

    println!("Part A: {}", parsed.iter().sum::<u32>())
}
