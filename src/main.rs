use std::{fs::File, io::{BufReader, self}, collections::HashMap};
use clap::{Parser};

mod letter_occurrence;
mod most_common_substring;
mod do_both;

use crate::most_common_substring::common_substring;
use crate::do_both::do_both;


//Add more documentation
#[derive(Parser)]
struct CLI {
    //PathBuf is like a string but for file paths and work cross platform
    path: std::path::PathBuf,
}

fn main() {
    let args:CLI = CLI::parse();

    let get_file = File::open(&args.path);

    let file = match get_file {
        Ok(file) => {file},
        Err(error) => panic!("Cant deal with this error, {}",error)
    };

    let file = BufReader::new(file);

    let common = common_substring(file,3);

    let gun = sort_map(common);

    println!("{:?}",gun);
    // let mut user_input = String::new();


    // println!("What do you want to do with this file?");
    // println!("1: Find the most common chars");
    // println!("2: Find the most common three-letter substrings");
    // println!("3: Do both!");

    // io::stdin().read_line(&mut user_input).expect("Couldnt read line");
    // let number: i32 = user_input.trim().parse().expect("Please type one of the given numbers");

    // if number == 1 {
    //     let (wc,occurrences) = letter_occurrence::letter_occurence(file);
    
    //     println!("The number of characters without spaces is: {}",&wc);

    //     print_occurences(&occurrences);

    //     println!("========================================");

    //     print_percentage(&occurrences, wc);
        
    // } else if number == 2 {
    //     let substrings = common_substring(file,3);

    //     println!("{:?}",substrings);
    // } else if number == 3{
    //     let (wc,letter_occurrences,most_common_substring)
    //     = do_both(file,3);

        
    //     println!("The number of characters without spaces is: {}",&wc);

    //     print_occurences(&letter_occurrences);

    //     println!("========================================");

    //     print_percentage(&letter_occurrences, wc);

    //     println!("========================================");

    //     print_occurences(&most_common_substring);

    // }
    
   
}

fn print_occurences<T>(occurrence: &HashMap<T, u32>) 
    where
    T: std::fmt::Display,
{
    for (key, value) in occurrence {
        println!("{}: {}", key, value);
    }
}

fn print_percentage(occurrence: &HashMap<char, u32>,wc:u32){
    for (char,val) in occurrence{
        let fug:f64 = per_cent(wc, &val);
        println!("{}: {:.2} %",char,fug);
    }
}

fn per_cent(whole: u32,part: &u32) -> f64{
    (*part as f64 / whole as f64) * 100.0
}


//Takes a hashmap and sorts it based on the values. 
//It is the return as a vec with tuples with key as the first value and value as second 
fn sort_map<T>(map: HashMap<T,u32>) -> Vec<(T,u32)>
//Makes sure that the generic value implements these properties
where 
T: std::fmt::Display + Ord,
{
    //This makes a new vector with the key value pairs as tuples. Map creates an iterator over the pairs
    //The collect method converts the iterator to a Vec<(T, u32)>
    let mut sorted_map: Vec<(T, u32)> = map.into_iter().collect();
    //The sort_by method takes a comparator to order the pairs based on the values 
    sorted_map.sort_by(|a, b| a.1.cmp(&b.1));
    //Reverses the vector so that the highest values come first and the smallest last
    sorted_map.reverse();
    sorted_map
}
