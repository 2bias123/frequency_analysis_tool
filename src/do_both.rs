use std::fs::File;
use std::collections::{HashMap};
use std::io::{BufReader,BufRead};

use crate::most_common_substring::substrings_in_line;
use crate::letter_occurrence::update_letter_count;


//This iterates over the file and counts the occurence of every char and finds the most common substrings of a given length
pub fn do_both(content: BufReader<File>, substring_length: usize) -> (u32, HashMap<char, u32>, HashMap<std::string::String, u32>){
    let mut letter_occurrences: HashMap<char,u32> = HashMap::new();
    let mut most_common_substring: HashMap<String, u32> = HashMap::new();
    let mut wc:u32 = 0;

    for line in content.lines(){
        match line {
            Ok(line) => {
                let (new_wc,new_letter_occurrence) 
                = update_letter_count(Some(&line), Some(letter_occurrences), wc);
                most_common_substring = substrings_in_line(Some(&line), Some(most_common_substring), substring_length);
                wc = new_wc;
                letter_occurrences = new_letter_occurrence;
            },
            Err(error) => println!("Couldnt read line due to this {}",error),
        }
    }
    (wc,letter_occurrences,most_common_substring)
}