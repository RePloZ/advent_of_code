use std::path::Path;
use std::fs;

fn main() {
    // --snip--
    let file_path = Path::new("../data/day_1_input.txt");
    println!("In file {}", file_path.display());

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    
    let result = contents.lines()
        .fold(0, |acc, line| {
            let number = line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<_>>();
            let first_number = number.first().map(|c| c.to_digit(10)).flatten().unwrap_or(0) as i32;
            let last_number = number.last().map(|c| c.to_digit(10)).flatten().unwrap_or(0) as i32;
            println!("Current line {line} - first number {first_number} - last number {last_number}");
            acc + first_number * 10 + last_number
        });
    println!("With text:\n{result}");
}