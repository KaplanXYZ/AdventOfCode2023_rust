// Alexander Kapllanaj Advent of Code day 1, second part

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
            panic!("Errore nell'aperta del file: {}", error); // If can't read panic (program stops)
        }
    };
    let reader: BufReader<File> = BufReader::new(input_file); // BufReader for reading file lines
    let mut input_txt: Vec<String> = vec![]; // Vector where i put all lines
    for line in reader.lines() {
        input_txt.push(line.unwrap()); // Push file lines into vector
    }
    let mut sum: u32 = 0; // Sum variable, this is the answer to the problem
    let mut n; // Variable to keep track of the index of the line
    let mut real_n; // Variable to keep track of the index I'm starting my temporary strings (non so come spiegarlo)
    let mut temp_string: String; // Temporary string to check if it's a number, if doesn't corrispond to any number it reset to ""

    for line in input_txt {
        n = 0; // Initializing variables
        real_n = 0;
        temp_string = "".to_string();
        sum += loop {
            // sum variable adds value from loop
            // Loop the line till i find a number (integer or written)
            let c: char = line.chars().nth(n).unwrap(); // Take char at the n position (from first to last)
            if c.is_digit(10) {
                break c.to_digit(10).unwrap() * 10; // When i get a number break the loop and add the number to my sum variable (the * 10 is a shortcut, I don't concatenatethe 2 numbers and then parse them to int)
            } else {
                temp_string.push(c); // If i don't find a number I add the char to my temporary string to check if I found a written number
                match check_if_number_is_str(temp_string, false) {
                    // I start my function
                    (true, s) => break s.1 * 10, // If temporary string corresponds to a written number i break the loop and add value to sum
                    (false, s) => {
                        // If temporary string doesn't correspond to a full written number but it's part of a written number the loop continues and checks next character of the line
                        if s.1 == 1 {
                            n += 1;
                        } else {
                            real_n += 1; // If temporary string it's not present in any written number i restart the loop starting from the next letter
                            n = real_n; // N becomes real_n because I don't want the loop to start from the same char but the next one
                        }
                        temp_string = s.0; // Temporary string gets value that function returns, it remains the same or becomes ""
                    }
                };
            }
        };
        n = 0; // Reset every variable to 0, checks backwards now
        real_n = 0;
        temp_string = "".to_string();
        sum += loop {
            // sum variable adds value from loop
            let c: char = line.chars().nth(line.len() - n - 1).unwrap(); // Take chars from last to first
            if c.is_numeric() {
                break c.to_digit(10).unwrap(); // When i get a number break the loop and add the number to my sum variable/
            } else {
                temp_string.push(c); // If i don't find a number I add the char to my temporary string to check if I found a written number
                match check_if_number_is_str(temp_string, true) {
                    // I start my function
                    (true, s) => {
                        break s.1; // If temporary string corresponds to a written number i break the loop and add value to sum
                    }
                    (false, s) => {
                        // If temporary string doesn't correspond to a full written number but it's part of a written number the loop continues and checks next character of the line
                        if s.1 == 1 {
                            n += 1;
                        } else {
                            real_n += 1; // If temporary string it's not present in any written number i restart the loop starting from the next letter
                            n = real_n; // N becomes real_n because I don't want the loop to start from the same char but the next one
                        }
                        temp_string = s.0; // Temporary string gets value that function returns, it remains the same or becomes ""
                    }
                };
            }
        }
    }
    println!("The sum is: {}", sum); // Answer
    Ok(())
}

fn check_if_number_is_str(temp_string: String, reversed: bool) -> (bool, (String, u32)) {
    // Function that checks if the string given is a written number
    let numbers = [
        // Array of tuples (Possible strings, their value)
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for line in numbers {
        // Iterate through the numbers array
        if !reversed {
            // Parameter reverted says if I'm checking my string going from first to last or from last to first
            match line.0.find(&temp_string) {
                // .find() returns Some(x) if it finds the string given in another string with x being the first index where the string is encountered
                Some(value) => {
                    if value == 0 {
                        // If first index where temporary string is encountered isn't the first don't consider
                        if line.0 == temp_string {
                            // if temporary string correspond to a written number return true and the value of the number
                            return (true, (line.0.to_string(), line.1));
                        } else {
                            return (false, (temp_string, 1)); // If temporary strings is partially found return false and return temporary string, the loop will continue in main function
                        }
                    }
                }
                None => (), // If none just do nothing
            }
        } else {
            // If i'm checking from right to left the strings need to be reversed
            match line.0.chars().rev().collect::<String>().find(&temp_string) {
                // .find() returns Some(x) if it finds the string given in another string with x being the first index where the string is encountered
                Some(value) => {
                    if value == 0 {
                        // If first index where temporary string is encountered isn't the first don't consider
                        if line.0.chars().rev().collect::<String>() == temp_string {
                            // if temporary string correspond to a written number return true and the value of the number
                            return (true, (line.0.to_string(), line.1));
                        } else {
                            // If temporary strings is partially found return false and return temporary string, the loop will continue in main function
                            return (false, (temp_string, 1));
                        }
                    }
                }
                None => (), // If none just do nothing
            }
        }
    }
    return (false, ("".to_string(), 0)); // If temporary strings isn't found return false and a empty string, loop in main function will restart from next letter
}
