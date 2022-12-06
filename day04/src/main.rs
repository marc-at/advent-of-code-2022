use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn make_list(dashed: &str) -> Vec<i32> {
    let parts: Vec<&str> = dashed.split("-").collect();
    let start = parts[0].parse::<i32>().unwrap();
    let end = parts[1].parse::<i32>().unwrap();
    let mut full: Vec<i32> = vec![start];
    let mut x = start + 1;
    while x <= end {
        full.push(x);
        x += 1;
    }
    return full;
}

fn main() {
    println!("Day 04!");

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut total1: usize = 0;
    let mut total2: usize = 0;
    let file_contents = read_lines(path);
    if let Ok(lines) = &file_contents {
        for line in lines {
            let parts: Vec<&str> = line.split(",").collect();
            let list0 = make_list(parts[0]);
            let list1 = make_list(parts[1]);
            if list0.iter().all(|e| list1.contains(e)) || list1.iter().all(|e| list0.contains(e)) {
                total1 += 1;
            }
            if list0.iter().any(|e| list1.contains(e)) {
                total2 += 1;
            }
        }
    }
    println!("total = {:?}", total1);
    println!("total = {:?}", total2);
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|i| i.unwrap()).collect::<Vec<String>>())
}
