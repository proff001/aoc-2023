fn main() {
    let input = std::fs::read_to_string("./inputs/day04.ex.txt").unwrap();

    let part_a: u32 = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();

            let all_numbers: Vec<&str> = split[1].split("|").collect();
            let mut points = 0;

            println!("{}", all_numbers[0].trim());
            println!("{}", all_numbers[1].trim());

            let winning: Vec<&str> = all_numbers[0].trim().split(" ").collect();
            let numbers = all_numbers[1].trim().split(" ").map(|num| num.trim());

            for num in numbers {
                if winning.iter().any(|w| w == &num) {
                    if points < 1 {
                        points += 1;
                    } else {
                        points *= 2;
                    }
                };

                println!("{num}, {points}");
            }

            points
        })
        .sum();

    let part_b: u32 = 32;

    println!("Part A: {part_a}");
    println!("Part B: {part_b}");
}
