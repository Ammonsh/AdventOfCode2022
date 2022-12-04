use std::time::{Instant};
use std::fs::{File};
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let start = Instant::now();

    let mut total_pairs_in_range = 0;
    let mut overlapping_pairs = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split(",");

        // initialize array of size 4
        let mut arr: [i32; 4] = [0; 4];

        // loop through split and enumerate
        for (i, s) in split.enumerate() {
            let split2 = s.split("-");
            arr[i*2] = split2.clone().nth(0).unwrap().parse::<i32>().unwrap();
            arr[i*2 + 1] = split2.clone().nth(1).unwrap().parse::<i32>().unwrap();
        }
        
        // Check if pair 1 is in range of pair 2 or vise versa
        if arr[0] <= arr[2] && arr[1] >= arr[3] || arr[2] <= arr[0] && arr[3] >= arr[1]{
            total_pairs_in_range += 1;
        }

        // Check if pair 1 overlaps pair 2 or vise versa
        if arr[1] >= arr[2] && arr[0] <= arr[3] || arr[3] >= arr[0] && arr[2] <= arr[1]{
            overlapping_pairs += 1;
        }

    }

    println!("Part 1: {}", total_pairs_in_range);
    println!("Part 2: {}", overlapping_pairs);

    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Ran for {} miliseconds", elapsed);
}


