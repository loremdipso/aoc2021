use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_pieces() -> Vec<Vec<String>> {
    let mut rv: Vec<Vec<String>> = vec![];
    let mut current: Vec<String> = vec![];
    if let Ok(lines) = read_lines(&env::args().skip(1).collect::<Vec<String>>()[0]) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    current.push(line.parse::<String>().unwrap());
                } else {
                    rv.push(current);
                    current = vec![];
                }
            }
        }
    }

    rv.push(current);
    return rv;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
