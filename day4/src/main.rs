// Parses the file from a string and populates struct
fn parse(input: &str) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let split = input.lines();
    let results: Vec<&str> = split.collect();

    let numbers = results[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // Temp vector, Hold the rows until it gets cleared
    let mut rows: Vec<Vec<u32>> = Vec::new();
    let mut boards = Vec::new();

    for e in &results[2..] {
        if *e != "" {
            let l: Vec<u32> = e.trim()
                .replace("  ", " ")
                .split(" ")
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

                rows.push(l);
        } else {
            boards.push(rows.clone());
            rows.clear();
        }
    }
    // Push the last board to the vector
    boards.push(rows);

    (numbers, boards)
}

fn mark_board(board: &mut Vec<Vec<u32>>, number: u32) {
    for row in board {
        for x in row {
            if *x == number {
                // A value of 100 will indicate that the 
                //number in that position has been marked
                *x = 100;
            }
        }
    }
}

fn is_bingo(board: &Vec<Vec<u32>>) -> bool{
    // Check if any of the rows has a bingo
    for row in board {
        if row.iter().all(|r| *r == 100) {return true}
    }
    // Check if any of the columns has a bingo
    for c in 0..5 {
        let mut column: Vec<u32> = Vec::new();
        for r in 0..5 {
            column.push(board[r][c]);
        }
        if column.iter().all(|r| *r == 100) {return true}
    }
    false
}


fn get_sum(board: &Vec<Vec<u32>>) -> u32{
    let mut sum = 0;

    for row in board {
        for x in row {
            if *x != 100 {
                sum += *x;
            }
        }
    }

    sum
}

fn main() {
    // Read file to string
    let input = include_str!("../input.txt");

    // Parse input
    let (numbers, mut boards) = parse(&input);

    let mut winner_indexes: Vec<usize> = Vec::new();
    let mut winners: Vec<u32> = Vec::new();

    // Iterate over all the bingo numbers
    for number in numbers {
        
        // For each number we will iterate over all the boards
        for i in 0..boards.len() {

            mark_board(&mut boards[i], number);
            
            if is_bingo(&boards[i]) && !winner_indexes.contains(&i) {
                let sum = get_sum(&boards[i]);

                winners.push(sum* number);

                winner_indexes.push(i);
            }
        }
    }

    println!("[Part 1] First bingo result: {:?}", winners[0]);
    println!("[Part 2] Last bingo result: {:?}", winners[winners.len() - 1]);
}