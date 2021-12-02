// https://adventofcode.com/2021/day/2

fn main() {
    // Read file to string
    let input = include_str!("../input.txt");

    // Turn string to vector
    let split = input.split("\n");
    let results: Vec<&str> = split.collect();

    // Get first solution (1604850 from my data)
    let solution1 = part1(&results);
    println!("[Part 1] Multiplying the final horizontal position the depth gives us: {}", solution1);

    // Get second solution (1685186100 from my data)
    let solution2 = part2(&results);
    println!("[Part 2] Multiplying the final horizontal position the depth gives us: {}", solution2);
}

// This function tokenizes the command string
fn tokenize(input: &str) -> (&str, u32) {
    // Split the result into tokens. "up 5" becomes "up" and "5"
    let mut tokens = input.split_ascii_whitespace();

    // Get the command from the tokens string
    let command = tokens
        .next()
        .unwrap();

    // Get the value of the command and parse to u32
    // We use u32 because the final result is pretty large
    let value = tokens
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // Return these values in a tuple
    return (command, value);
}

fn part1(results: &Vec<&str>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for result in results {
        let (command, value) = tokenize(&result);

        // Match the string with the possible commands. Panic if no mach is found
        match command {
            "forward" => {
                // Forward ads its value to the horizontal position
                horizontal += value;
            },
            "up" => {
                // Up subtracts its value from the depth
                depth -= value;
            },
            "down" => {
                // Depth ads its value to the depth
                depth += value;
            },
            _ => panic!("Unknown command! {}", command)
        }
    }
    
    // Multiply the horizontal position and depth and return
    return horizontal * depth;
}

fn part2(results: &Vec<&str>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    let mut aim = 0;
    for result in results {
        let (command, value) = tokenize(&result);

        // Match the string with the possible commands. Panic if no mach is found
        match command {
            "forward" => {
                // Forward ads its value to the horizontal position
                horizontal += value;
                // Forward ads its value multiplied by the aim to the depth
                depth += value*aim;
            },
            "up" => {
                // Up subtracts its value from the aim
                aim -= value;
            },
            "down" => {
                // Depth ads its value to the aim
                aim += value;
            },
            _ => panic!("Unknown command! {}", command)
        }
    }
    
    // Multiply the horizontal position and depth and return
    return horizontal * depth;
}