use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let path = &args[1];

    let mut max: Vec<i32> =  Vec::new();
    if let Ok(lines) = read_lines(path) {
        let mut current = 0;
        for line in lines {
            let my_int = line.unwrap().parse::<i32>().unwrap_or(0);
            if my_int == 0 {
                if max.len() == 0 || current >= max[0] {
                    max.insert(0, current);
                } else {
                    max.push(current);
                }
                current = 0
            } else {
                current = current + my_int
            }
        }
    }
    
    println!("max = {}", max[0]);
    
    max.sort();

    let y: Vec<_> = max
        .into_iter()
        .rev()
        .collect();
        
    println!("top 3 = {:?}", &y[0..3].into_iter().sum::<i32>());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// correct = 67633