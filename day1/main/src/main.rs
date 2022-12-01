use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    // Split the input into lines
    let lines: Vec<&str> = input.lines().collect();  

    // Initialize empty vector of type i32 to store sums
    let mut sums: Vec<i32> = Vec::new();

    let mut curr_sum: i32 = 0;

    // Iterate over the lines
    for line in lines {
        // Check if the line is an empty string
        if line.is_empty() {
            // Add the current sum to the vector of sums
            sums.push(curr_sum);
            curr_sum = 0;
            continue;
        }
        // Convert the line to an i32
        let num: i32 = line.parse().unwrap();

        // Add the number to the current sum
        curr_sum += num;
    }

    part1(&sums);
    part2(&sums);

}

// Part 1 function
fn part1(sums: &Vec<i32>){
    // Print max sum in sums
    println!("{}", sums.iter().max().unwrap());
}

fn part2(sums: &Vec<i32>) {
    // get max 3 sums
    let mut max_sums: Vec<i32> = sums.iter().cloned().collect();
    max_sums.sort();
    max_sums.reverse();
    max_sums.truncate(3);

    // sum max_sums
    let sum: i32 = max_sums.iter().sum();
    
    // print sum
    println!("{}", sum);
}