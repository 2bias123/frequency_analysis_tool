use std::fs::File;
use std::collections::{BTreeMap};
use std::io::{BufReader,BufRead};

//Function that reads a buffreader file and finds the most common substrings with a given length
pub fn common_substring(content: BufReader<File>, substring_length: usize) -> BTreeMap<std::string::String, i32>{
    let mut letter_occurence = BTreeMap::new();
    for line in content.lines(){
        //If the line was succsessfully read sends it for the helper function for processing
        match line {
            Ok(line) => {
                letter_occurence = substrings_in_line(line, Some(letter_occurence), substring_length);
            },
            //Prints error if the line couldnt be read
            Err(error) => println!("Couldnt read line due to this {}",error),
        }
    }
    return letter_occurence;
}

//Finds every substring with a given length in a single line and records them in a map
fn substrings_in_line(line: String, input_map: Option<BTreeMap<String, i32>>, substring_length: usize) -> BTreeMap<String, i32>{

    //Unpacks the input_map option and checks if it contains a map
    //if it does it sets input_map to that map
    //And if it dosent it initiates a new map
    let mut map = match input_map {
        Some(map) => map,
        None => BTreeMap::new(),
    };

    //This code iterates over a string line and counts the number of occurrences 
    //of substrings of a certain length (substring_length) within the string.
    for i in 0..line.len() - substring_length + 1 {
        let substr = &line[i..i+substring_length];
        *map.entry(substr.to_string()).or_insert(0) += 1;
    }

    return map; 
}