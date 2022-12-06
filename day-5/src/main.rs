use std::time::{Instant};
use std::fs::{File};
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input_moves.txt").unwrap();
    let reader = BufReader::new(file);

    let start = Instant::now();

    // Part 1
    let mut stacks_part_1 = initialize_stacks();
    let mut stacks_part_2 = initialize_stacks();

    // Read input_moves.txt
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();

        line.next().unwrap();
        let move_val:i32 = line.next().unwrap().parse().unwrap();
        line.next().unwrap();
        let from_val:i32 = line.next().unwrap().parse().unwrap();
        line.next().unwrap();
        let to_val:i32 = line.next().unwrap().parse().unwrap();

        // Part 1
        // Move mov_val items from from_val stack to to_val stack
        for _i in 0..move_val {
            let item = stacks_part_1[(from_val as usize)-1].pop();
            stacks_part_1[(to_val as usize)-1].push(item.unwrap() as char);
        }

        // Part 2
        // Move mov_val items from from_val stack to to_val stack, but do it in order.
        let mut temp_stack:Vec<char> = Vec::new();

        for _i in 0..move_val {
            let item = stacks_part_2[(from_val as usize)-1].pop();
            temp_stack.push(item.unwrap() as char);
        }
        
        // push items from temp_stack to to_val stack
        for _i in 0..move_val {
            let item = temp_stack.pop();
            stacks_part_2[(to_val as usize)-1].push(item.unwrap() as char);
        }
    
    }
    
    // print out the last item of each stack for part 1
    println!("Part 1:");
    for i in 0..stacks_part_1.len() {
        print!("{}",stacks_part_1[i].last().unwrap());
    }
    println!("");
   
    // print out the last item of each stack for part 2
    println!("Part 2:");
    for i in 0..stacks_part_2.len() {
        print!("{}",stacks_part_2[i].last().unwrap());
    }
    println!("");


    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Ran for {} miliseconds", elapsed);
}

fn initialize_stacks() -> Vec<Vec<char>> {
    let stack1:Vec<char> = vec!['R','G','J','B','T','V','Z'];
    let stack2:Vec<char> = vec!['J','R','V','L'];
    let stack3:Vec<char> = vec!['S','Q','F'];
    let stack4:Vec<char> = vec!['Z','H','N','L','F','V','Q','G'];
    let stack5:Vec<char> = vec!['R','Q','T','J','C','S','M','W'];
    let stack6:Vec<char> = vec!['S','W','T','C','H','F'];
    let stack7:Vec<char> = vec!['D','Z','C','V','F','N','J'];
    let stack8:Vec<char> = vec!['L','G','Z','D','W','R','F','Q'];
    let stack9:Vec<char> = vec!['J','B','W','V','P'];

    // Add all stacks to a vector
    let stacks:Vec<Vec<char>> = vec![stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9];
    stacks
}