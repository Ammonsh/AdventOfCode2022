use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let chars_1: &mut [char] = &mut ['\0', '\0', '\0', '\0'];
    let chars_2: &mut [char] = &mut ['\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'];

    find_marker(chars_1, &contents);
    find_marker(chars_2, &contents);

    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Ran for {} miliseconds", elapsed);
}

fn find_marker(chars: &mut [char], input: &str) {
    let mut count = 0;

    for c in input.chars() {
        chars[count % chars.len()] = c;

        if chars.contains(&'\0') {
            count += 1;
            continue;
        } else {
            let mut set = std::collections::HashSet::new();
            for c in chars.iter() {
                set.insert(c);
            }
            if set.len() == chars.len() {
                count += 1;
                println!("Count: {}", count);
                break;
            }
        }
        count += 1;
    }
}
