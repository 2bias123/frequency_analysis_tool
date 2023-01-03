use std::{fs::File, io::BufReader, collections::BTreeMap};
use clap::Parser;
use most_common_substring::common_substring;

mod letter_occurrence;
mod most_common_substring;

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

    // let substrings = common_substring(file,3);

    // println!("{:?}",substrings);

    let occurrences = letter_occurrence::letter_occurence(file);
    let wc:u32 = occurrences.0;
    println!("The number of characters without spaces is: {}",&wc);

    print_occurences(&occurrences.1);

    println!("========================================");

    print_percentage(&occurrences.1, wc);
   
}

fn print_occurences(occurrence: &BTreeMap<char, u32>) {
    for (key,value) in occurrence{
        println!("{key}: {value}");
    }
}

fn print_percentage(occurrence: &BTreeMap<char, u32>,wc:u32){
    for (char,val) in occurrence{
        let fug:f64 = per_cent(wc, &val);
        println!("{}: {:.2} %",char,fug);
    }
}

fn per_cent(whole: u32,part: &u32) -> f64{
    (*part as f64 / whole as f64) * 100.0
}