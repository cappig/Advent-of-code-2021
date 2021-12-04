// https://adventofcode.com/2021/day/3

fn main() {
    let input = include_str!("../input.txt");

    let lines = input.lines();
    let results: Vec<&str> = lines.collect();

    // Get first solution (2724524 from my data)
    let solution1 = part1(&results);
    println!("[Part 1] The product of gamma and epsilon is {}", solution1);

    // Get second solution (2775870 from my data)
    let solution2 = part2(&results);
    println!("[Part 2] The product of oxygen and carbon is {}", solution2);
}

// Convert a binary vector to a decimal number
fn to_decimal(binary: &[u32]) -> u32 {
    binary
        .iter()
        .fold(0, |acc, &b| acc * 2 + b)
}

fn most_dominant_bit(results: &Vec<&str>) -> Vec<u32> {
    // This is a vector that hold the sums of the columns
    // The bytes ae 12 bits long so we 12 columns
    let mut columnsum = vec![0; 12];

    // Iterate over all the bytes in the vector
    for byte in results {
        // Convert the string to a vector of chars
        let bitvec: Vec<u32> = byte
            .chars()
            .map(|bit| bit.to_digit(2).unwrap())
            .collect();
        
        // Add the values of the columns to the sum
        for (i, c) in columnsum.iter_mut().enumerate() {
            *c += bitvec[i];
        }
    }
    columnsum
}

fn part1(results: &Vec<&str>) -> u32 {
    let columnsum = most_dominant_bit(results);
    println!("{:?}", columnsum);

    // The gamma and epsilon values are both stored in a vector
    let mut gamma = [0; 12];
    let mut epsilon = [0; 12];

    // Iterate over all the columns
    for i in 0..12 {
        // If the sum is larger than half od the total number of 
        // bytes (number of bytes is 1000, half of that is 500) 
        // there are more ones.
        // Explained here: https://bit.ly/3rxGlvL

        // Gamma is the most common bit, epsilon is the least common bit
        if columnsum[i] >= 500 {
            gamma[i] = 1;
            epsilon[i] =0;
        } else {
            gamma[i] = 0;
            epsilon[i] = 1;
        }
    }

    // Convert to decimal and multiply gamma and epsilon
    to_decimal(&epsilon) * to_decimal(&gamma)
}

fn part2(results: &Vec<&str>) -> u32 {

    // part2calc takes the result and a boolean variable
    // if the bool is true that means we are looking for
    // the oxygen value, if ist false we're looking for carbon
    let oxygen = part2calc(&results, true);
    let carbon = part2calc(&results, false);

    // Multiply and return
    oxygen * carbon
}

fn part2calc(results: &Vec<&str>, oxygen: bool) -> u32 {
    let mut mutresults = results.to_owned();

    let mut i = 0;
    // Loop until we are left with only one element
    while mutresults.len() > 1 {
        let columnsum = most_dominant_bit(&mutresults);

        // The number of zeros is equal to the entire length - number of ones
        let zeros= mutresults.len() as u32 -columnsum[i];

        if columnsum[i] >= zeros {
            mutresults.retain(|bit| {
                // If we want the oxygen value we need the current bit 
                // to be equal to the dominant bit of the whole column
                if oxygen {
                    bit[i..12].starts_with('1')
                } 
                // If were looking fr the carbon value we need the bit
                // t obe equal to the least common bit
                else { 
                    bit[i..12].starts_with('0')
                }
            });
        } else {
            mutresults.retain(|bit| {
                if oxygen { 
                    bit[i..12].starts_with('0')
                } else {
                    bit[i..12].starts_with('1')
                }
            });
        }
        i += 1;
    }

    // Convert to decimal
    u32::from_str_radix(mutresults[0], 2).unwrap()
}