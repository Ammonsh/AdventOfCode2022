use std::time::{Instant};
use std::char;
use std::collections::HashSet;

fn main() {
    // create empty vector of i32
    let mut v: Vec<i32> = Vec::new();
    let mut first_half: &str;
    let mut second_half: &str;
    let mut common_letter: char;
    let mut priority: i32;
    
    // read input into vector
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = Instant::now();

    // Part 1, get the sum of all priorities
    for line in input.lines() {
        (first_half, second_half) = split_string_in_half(line);
        common_letter = find_common_letter(first_half, second_half);
        priority = get_priority_of_character(&common_letter);
        v.push(priority);
    }

    println!("Sum: {}", v.iter().sum::<i32>());

    // Part 2, find the sum of the priorities based on groups of 3
    // Loop through the vector in groups of 3
    let mut sum: i32 = 0;
    let mut lines = input.lines();
    loop {
        let first = if let Some(i) = lines.next() {
            i
        }
        else {
            break;
        };
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        
        let common_letters: HashSet<char> = find_common_letters(first, second, third);
        for letter in common_letters {
            sum += get_priority_of_character(&letter);
        }
    }
    println!("Sum: {}", sum);


    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Ran for {} miliseconds", elapsed);
}

fn get_priority_of_character(c: &char) -> i32 {
    let ascii_val: i32 = *c as i32;
    let priority: i32;

    if ascii_val > 64 && ascii_val < 91 {
        priority = ascii_val - 38;
    } else if ascii_val > 96 && ascii_val < 123 {
        priority = ascii_val - 96;
    } else {
        priority = 0;
    }
    
    priority
}

fn split_string_in_half(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    s.split_at(mid)
}

fn find_common_letter(first_half: &str, second_half: &str) -> char {
    let mut common_letter = ' ';

    // create sets for first and second halfs of strings
    let first_half_set: std::collections::HashSet<char> = first_half.chars().collect();
    let second_half_set: std::collections::HashSet<char> = second_half.chars().collect();

    // find common letter by checking the intersection of the sets
    for letter in first_half_set.intersection(&second_half_set) {
        common_letter = *letter;
    }

    common_letter
}

fn find_common_letters(str1: &str, str2: &str, str3: &str) -> HashSet<char> {
    let mut common_letters: HashSet<char> = HashSet::new();
    let mut temp_intersection: HashSet<char> = HashSet::new();

    // create sets for all three strings
    let str1_set: HashSet<char> = str1.chars().collect();
    let str2_set: HashSet<char> = str2.chars().collect();
    let str3_set: HashSet<char> = str3.chars().collect();

    // find common letters by checking the intersection of the sets
    for letter in str1_set.intersection(&str2_set) {
        temp_intersection.insert(*letter);
    }
    for letter in temp_intersection.intersection(&str3_set) {
        common_letters.insert(*letter);
    }

    common_letters
}