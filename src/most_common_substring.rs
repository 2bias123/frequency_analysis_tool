use std::fs::File;
use std::collections::{BTreeMap};
use std::io::{BufReader,BufRead};

pub fn common_substring(content: BufReader<File>, substring_length: usize) -> BTreeMap<std::string::String, i32>{
    let mut letter_occurence = BTreeMap::new();
    for line in content.lines(){
        match line {
            Ok(line) => 
            for i in 0..line.len() - substring_length + 1 {
                let substr = &line[i..i+substring_length];
                *letter_occurence.entry(substr.to_string()).or_insert(0) += 1;
            } ,
            Err(_) => todo!(),
        }
    }
    return letter_occurence;
}

// fn divide_string(s: &str, length: usize,map: Result<BTreeMap<String, usize>> ) -> BTreeMap<String, usize> {
//     let mut map = BTreeMap::new();
//     for i in 0..s.len() - length + 1 {
//         let substr = &s[i..i+length];
//         let count = map.entry(substr.to_string()).or_insert(0);
//         *count += 1;
//     }
//     map
// }