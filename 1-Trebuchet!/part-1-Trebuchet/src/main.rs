// Alexander Kapllanaj Advent of Code day 1, first part

use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error}, // Libraries
};

fn main() -> Result<(), Error> {
    // Main function
    let input_file: File = match File::open("./input.txt") {
        // Read input file
        Ok(file) => file,
        Err(error) => {
            // If can't read panic (program stops)
            panic!("Errore nell'aperta del file: {}", error);
        }
    };
    let reader: BufReader<File> = BufReader::new(input_file); // BufReader for reading file lines
    let mut input_txt: Vec<String> = vec![]; // Vector where i put all lines
    for line in reader.lines() {
        input_txt.push(line.unwrap()); // Push file lines into vector
    }
    let mut sum: u32 = 0; // Sum variable, this is the answer to the problem
    let mut n; // Variable to keep track of the index of the line
    for line in input_txt {
        n = 0;
        sum += loop {
            // sum variable adds value from loop
            // Loop where i check every line till i get a number
            let c: char = line.chars().nth(n).unwrap(); // Getting chars from first to last
            if c.is_digit(10) {
                break c.to_digit(10).unwrap() * 10; // When i get a number break the loop and add the number to my sum variable (the * 10 is a shortcut, I don't concatenatethe 2 numbers and then parse them to int)
            } else {
                n += 1; // If no number found increment n to check next index of the line
            }
        };
        n = 0; // Reset n to 0, i'm gonna repeat the process but backwards
        sum += loop {
            // sum variable adds value from loop
            let c: char = line.chars().nth(line.len() - n - 1).unwrap(); // Getting chars from last to first
            if c.is_digit(10) {
                break c.to_digit(10).unwrap(); // When i get a number break the loop and add the number to my sum variable
            } else {
                n += 1; // If no number found increment n to check next index of the line
            }
        }
    }
    println!("The sum is: {}", sum); // Answer
    Ok(())
}
