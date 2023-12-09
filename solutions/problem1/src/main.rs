use regex::Regex;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut total = 0;

    // Open the file in read-only mode
    let file_result = std::fs::File::open("/home/isaiah/git/advent_of_code/input/problem1.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            return Ok(());
        }
    };

    let reader = io::BufReader::new(file);

    // Define a regular expression pattern to match numeric characters
    let re = Regex::new(r"\d+").unwrap();

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;
        // Perform your processing on each line here
        println!("Processing line: {}", line);

        // Find and concatenate all matched numeric characters in the line
        let concatenated: String = re.find_iter(&line).map(|mat| mat.as_str()).collect();

        println!("Concatenated numeric characters: {}", concatenated);

        let first_num = concatenated.chars().next(); // Get the first character
        let last_num = concatenated.chars().last(); // Get the last character

        if let (Some(first), Some(last)) = (first_num, last_num) {
            let concatenated_result = format!("{}{}", first, last); // Concatenate the characters
            if let Ok(concatenated_result_as_int) = concatenated_result.parse::<i32>() {
                total += concatenated_result_as_int;
            } else {
                println!("Conversion to integer failed");
            }
        } else {
            println!("Empty string or string with only one character");
        }
    }
    println!("Grand total: {}", total); // Concatenate the characters
    Ok(())
}
