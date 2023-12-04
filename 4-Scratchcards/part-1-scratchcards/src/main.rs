use core::panic;

use std::{collections::BTreeMap, fs::read_to_string, io::Error}; // Libraries

fn main() -> Result<(), Error> {
    let input_txt = read_to_string("./input.txt").expect("Should have been able to read the file"); // Read file
                                                                                                    // Variables
    let mut answer: i32 = 0; // Result variable

    for line in input_txt.lines() {
        // Iterating through the lines of the input file
        let mut matching_numbers = 0; // Variable to store matching numbers
        let mut map: BTreeMap<&str, bool> = BTreeMap::new(); // Map to store winning numbers
        let card: Vec<&str> = line.split(':').collect(); // Splitting the line
        let all_numbers: Vec<&str> = card[1].split('|').collect(); // Getting all the numbers
        let winning_numbers: Vec<&str> = all_numbers[0].split(' ').collect(); // Getting the winning numbers from all the numbers (it takes also some empty "" chars)
        let your_numbers: Vec<&str> = all_numbers[1].split(' ').collect(); // Getting the numbers i have
        for number in winning_numbers {
            // Iterating through winning numbers
            if number != "" {
                // If number is not an empty str insert it in a map
                map.insert(number, true);
            }
        }
        for number in your_numbers {
            // If number i have is present in the map add 1 to matching numbers
            if map.contains_key(number) {
                matching_numbers += 1;
            }
        }

        if matching_numbers != 0 {
            // If matching numbers is not equal to 0 (found at least 1 matching number)
            let base: i32 = 2; // Base set to 2
            answer += base.pow(matching_numbers - 1); // Add to answer the 2 at the power of matching_numbers - 1
        }
    }
    println!("{} is the answer", answer); // Print the answer
    Ok(())
}
