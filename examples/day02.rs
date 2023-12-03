fn main() {
    let input = std::fs::read_to_string("./inputs/day02.txt").unwrap();

    let part_a: u32 = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();

            let game = split[0]
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let revealed = split[1].split(";");
            let mut valid = true;

            for str in revealed {
                let cubes = str.split(",");

                for color in cubes {
                    let number = color
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();

                    if color.contains("red") {
                        if number > 12 {
                            valid = false;
                        }
                    } else if color.contains("green") {
                        if number > 13 {
                            valid = false;
                        }
                    } else if color.contains("blue") {
                        if number > 14 {
                            valid = false;
                        }
                    }
                }
            }

            if !valid {
                return 0;
            }

            game
        })
        .sum();

    let part_b: u32 = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();

            let revealed = split[1].split(";");

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for str in revealed {
                let cubes = str.split(",");

                for color in cubes {
                    let number = color
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();

                    if color.contains("red") {
                        if number > min_red {
                            min_red = number;
                        }
                    } else if color.contains("green") {
                        if number > min_green {
                            min_green = number;
                        }
                    } else if color.contains("blue") {
                        if number > min_blue {
                            min_blue = number;
                        }
                    }
                }
            }

            return min_red * min_green * min_blue;
        })
        .sum();

    println!("Part A: {part_a}");
    println!("Part B: {part_b}");
}
