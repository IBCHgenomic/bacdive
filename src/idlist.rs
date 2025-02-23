use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2024-2-23
*/

pub fn idlist(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut uniqueid: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") {
            let linesplit: Vec<_> = line.split(",").collect::<Vec<_>>();
            uniqueid.insert(linesplit[0].to_string());
        }
    }

    Ok(uniqueid)
}
