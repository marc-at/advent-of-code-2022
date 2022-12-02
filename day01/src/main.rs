use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let path = &args[1];

    if let Ok(lines) = read_lines(path) {
        let carlorie_list: Vec<i32> = lines
            .map(|e| {
                 e.unwrap().parse::<String>().unwrap()
                 .split("\n")
                 .map(|s| s.parse::<i32>().unwrap_or(0))
                 .sum()
            })
            .collect();

        let calorie_groups: Vec<Vec<i32>> = carlorie_list
            .into_iter()
            .fold(Vec::new(), |mut acc, x| {
                if x == 0 || acc.is_empty() {
                    acc.push(Vec::new());
                }
                acc.last_mut().unwrap().push(x);
                acc
            });

        let mut carorie_totals: Vec<i32> = calorie_groups
            .into_iter()
            .map(|o| {
                o.iter().sum::<i32>()
            })
            .collect();

        carorie_totals.sort();

        carorie_totals = carorie_totals
            .into_iter()
            .rev()
            .collect();

        println!("{:?}", carorie_totals[0]);
        println!("top 3 = {:?}", &carorie_totals[0..3].into_iter().sum::<i32>());

    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// correct = 67633