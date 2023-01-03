use std::fs::File;
use std::collections::{BTreeMap};
use std::io::{BufReader,BufRead};

//Function that reads a buffreader file and finds the most common substrings with a given length
pub fn common_substring(content: BufReader<File>, substring_length: usize) -> BTreeMap<String, u32>{
    let mut most_common_substring:BTreeMap<String, u32> = BTreeMap::new();


    for line in content.lines(){
        //If the line was succsessfully read sends it for the helper function for processing
        match line {
            Ok(line) => {
                most_common_substring = substrings_in_line(Some(&line), Some(most_common_substring), substring_length);
            },
            //Prints error if the line couldnt be read
            Err(error) => println!("Couldnt read line due to this {}",error),
        }
    }
    return most_common_substring;
}

//Finds every substring with a given length in a single line and records them in a map
pub fn substrings_in_line(line: Option<&String>, input_map: Option<BTreeMap<String, u32>>, substring_length: usize) -> BTreeMap<String, u32>{

    //Unpacks the input_map option and checks if it contains a map
    //if it does it sets input_map to that map
    //And if it dosent it initiates a new map
    let mut map = match input_map {
        Some(map) => map,
        None => BTreeMap::new(),
    };

    //This iterates over a string line and counts the number of occurrences 
    //of substrings of a certain length within the string.
    match line {
        Some(line) =>  for i in 0..line.len() - substring_length + 1 {
            let substr = &line[i..i+substring_length];
            if !substr.contains(" "){
                *map.entry(substr.to_string()).or_insert(0) += 1;
            }
        },
        None => println!("There wasnt any line to read"),
    }
   

    return map; 
}