use core::panic;
use std::cmp::max;
use std::{fs::read_to_string, io::Error}; // Libraries

fn main() -> Result<(), Error> {
    let input_txt = read_to_string("./input.txt").expect("Should have been able to read the file"); // Read file
                                                                                                    // Variables
    let mut answer: i32 = 0; // Result variable
    let mut power_of_game: i32 = 0; // Variable to store power of a game
    for line in input_txt.lines() {
        // Iterating through the lines of the input file
        let game: Vec<&str> = line.split(':').collect(); // Splitting line
        let rounds: Vec<&str> = game[1].split(';').collect(); // Splitting game Vec in a Vec of rounds
        let mut max_red = 0; // Declaring and initializing variables to 0 every line cycle
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in rounds {
            // Iterating through the rounds
            let cubes: Vec<&str> = round.split(',').collect(); // Splitting rounds Vec and collecting in cubes Vec
            for cube in cubes {
                // Iterating through the cubes
                let values: Vec<&str> = cube.split(' ').collect(); // Splitting cube Vec and collecting in values Vec, values Vec has 2 values now an integer and a &str
                let integer: i32 = values[1].parse().unwrap(); // Get first value (integer)
                let color: &str = values[2]; // Get second value (&str)
                match color {
                    "red" => max_red = max(integer, max_red), // If color equals to "red" check if integer is greater than current max value of red

                    "green" => max_green = max(integer, max_green), // If color equals to "green" check if integer is greater than current max value of green

                    "blue" => max_blue = max(integer, max_blue), // If color equals to "blue" check if integer is greater than current max value of blue

                    &_ => panic!("Errore: colore non trovato"), // Panic if color isn't equal to anything
                }
            }
        }
        power_of_game = max_red * max_green * max_blue; // Power of game becomes all max values multiplied together
        answer += power_of_game; // Add power_of_game to answer
    }
    println!("{} is the answer", answer); // Print answer
    Ok(())
}
