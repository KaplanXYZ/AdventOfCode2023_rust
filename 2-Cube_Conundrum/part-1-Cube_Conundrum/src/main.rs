use core::panic;
use std::{fs::read_to_string, io::Error}; // Libraries

fn main() -> Result<(), Error> {
    let input_txt = read_to_string("./input.txt").expect("Should have been able to read the file"); // Read file
                                                                                                    // Variables
    let mut answer: i32 = 0; // Result variable
    let mut game_counter: i32 = 1; // Variable to count games
    let mut found: bool; // Boolean variable to know if i found an impossible game
    for line in input_txt.lines() {
        // Iterating through the lines of the input file
        let game: Vec<&str> = line.split(':').collect(); // Splitting line
        found = false; // Initiliazing found variable to false at every line iteration
        let rounds: Vec<&str> = game[1].split(';').collect(); // Splitting game Vec in a Vec of rounds
        for round in rounds {
            // Iterating through the rounds
            let cubes: Vec<&str> = round.split(',').collect(); // Splitting rounds Vec and collecting in cubes Vec
            for cube in cubes {
                // Iterating through the cubes
                let values: Vec<&str> = cube.split(' ').collect(); // Splitting cube Vec and collecting in values Vec, values Vec has 2 values now an integer and a &str

                let integer: i32 = values[1].parse().unwrap(); // Get first value (integer)
                let color: &str = values[2]; // Get second value (&str)

                found = match color {
                    // Variable found takes value that match returns
                    "red" => check_integer(integer, 12), // If color equals to "red" start check_integer function
                    "green" => check_integer(integer, 13), // If color equals to "green" start check_integer function
                    "blue" => check_integer(integer, 14), // If color equals to "blue" start check_integer function
                    &_ => panic!("Errore nel colore"),    // Panic if color isn't equal to anything
                };
                if found {
                    // If found is true go out of the cube for
                    break;
                };
            }
            if found {
                // If found is true go out of the round for
                break;
            };
        }
        if !found {
            // If found is false add the game id to the answer
            answer += game_counter;
        }
        game_counter += 1; // Game_counter adds 1 and goes to next cycle
    }
    println!("{} is the answer", answer); // Print answer

    Ok(())
}

fn check_integer(value: i32, value_to_overcome: i32) -> bool {
    // Function to check if value overcomes his max possible value
    if value > value_to_overcome {
        return true; // If value is overcomed return true
    } else {
        return false; // Else return false
    }
}
