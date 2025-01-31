use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn hmmcount(path: &str) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut functionhold: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#"){
          continue
        } else if !line.starts_with("#"){
        functionhold.push(line.split(" ").filter(|x| !x.is_empty()).collect::<Vec<_>>()[22].to_string());
    }
    }
    let functionhashset: HashSet<_> = functionhold.iter().collect::<HashSet<_>>();
    let mut countindex: Vec<(String, usize)> = Vec::new();
    for i in functionhashset.iter() {
        for j in functionhold.iter() {
            let mut countcount: usize = 0usize;
            if *i == j {
                countcount += 1
            }
            let tuplevec: (String, usize) = (i.to_string(), countcount);
            countindex.push(tuplevec);
        }
    }
    Ok::<Vec<(String, usize)>, Box<dyn Error>>(countindex)
}
