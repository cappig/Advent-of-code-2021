// https://adventofcode.com/2021/day/1

fn main() {
    // Read file to string
    let input = include_str!("../input.txt");

    // Turn string to vector
    let split = input.split("\n");
    let strings: Vec<&str> = split.collect();

    // Turn string vector to u16
    let results: Vec<u16> = strings.iter().flat_map(|x| x.parse()).collect();

    // Get first solution (1791 from my data)
    let solution1 = part1(&results);
    println!("[Part 1] There are {} sonar results larger than the previous result.", solution1);

    // Get second solution (1822 from my data)
    let solution2 = part2(&results);
    println!("[Part 2] There are {} sums larger than the previous sum", solution2);
}

fn part1(results: &Vec<u16>) -> u16 {
    let mut larger = 0;
    for n in 0..results.len() {
        // Skip first one
        if n == 0 { continue; }

        // Iterate counter if larger
        if results[n] > results[n - 1]  {larger += 1;}
    }
    
    return larger;
}

fn part2(results: &Vec<u16>) -> u16 {
    let mut sequence: Vec<u16> = vec![];

    for n in 0..results.len() {
        // Break the loop once there are no more 
        // three-measurement sliding window can be calculated
        if n == results.len() - 2 { break; }
        
        // Calculate the three-measurement sliding window
        let seq = results[n] + results[n+1] + results[n+2];
        // Push to the final sequence vector
        sequence.push(seq);
    }

    // Call first function to determine how many results 
    // are larger than the previous.
    let larger = part1(&sequence);

    return larger;
}