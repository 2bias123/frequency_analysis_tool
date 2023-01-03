use std::fs::File;
use std::collections::{BTreeMap};
use std::io::{BufReader,BufRead};


//This function takes a buffreader file and returns a tuple with the word count, 
//and a map containg each letter mapped to the number of times it occurs in the file
pub fn letter_occurence(content: BufReader<File>) -> (u32,BTreeMap<char, u32>) {
    let mut letter_occurrence:BTreeMap<char,u32> = BTreeMap::new();
    let mut wc:u32 = 0;
    
    for line in content.lines(){
        //If the line was sucsessfully read it calls the helper function to update the wc and the map
        match line {
            Ok(line) => {
                let (new_wc,new_letter_occurrence) =
                update_letter_count(Some(&line), Some(letter_occurrence), wc);
                wc = new_wc;
                letter_occurrence = new_letter_occurrence;
            },
            //If there was an error reading the line it will print this error message
            Err(error) => println!("Couldnt read line due to this {}",error),
        }
    }

    return (wc,letter_occurrence);
}

pub fn update_letter_count(line:Option<&String>, letter_occurrence_input: Option<BTreeMap<char,u32>>,wc: u32) -> (u32,BTreeMap<char, u32>){
    //Makes a mutable copy of the wordcount
    let mut wc = wc;

    //Unpacks the letter occurance option and checks if it contains a map
    //if it does it sets letter_occurrence_map to that map
    //And if it dosent it initiates a new map
    let mut letter_occurrence_map = match letter_occurrence_input {
        Some(map) => map,
        None => BTreeMap::new(),
    };
    //If the line option contains a line process it
    match line {
        Some(line) => {
            //Iterate over every char in the line 
            for i in line.to_lowercase().chars(){
                //If the char is not a witespace or newline update the map and the wordcount
                if !i.is_whitespace() {
                    *letter_occurrence_map.entry(i).or_insert(0) += 1;
                    wc +=1;
                }
            }
        }
        None => println!("There wasnt any line to read")
    }
    (wc,letter_occurrence_map)
      
}