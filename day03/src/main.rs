use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use itertools::Itertools;

const ALPHA: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

#[derive(Debug)]
struct ElfGroup {
    lines: Vec<String>
}

impl Rucksack {

    fn from_string(line: &str) -> Rucksack {
        let (c1, c2) = line.split_at(line.len() / 2);
        Rucksack {
            compartment1: c1.to_string(),
            compartment2: c2.to_string()
        }
    }

    fn score(&self) -> usize {
        let res:Vec<char> = self.compartment1.chars().filter(|&c| self.compartment2.contains(c)).unique().collect();
        let score = ALPHA.find(res[0]);
        return score.unwrap();
    }
}

impl ElfGroup {

    fn from_chunk(chunk: Vec<String>) -> ElfGroup {
        ElfGroup {
            lines: chunk
        }
    }

    fn score(&self) -> usize {
        let res:Vec<char> = self.lines[0]
            .chars()
            .filter(|&c| self.lines[1].contains(c))
            .collect::<Vec<char>>()
            .into_iter()
            .filter(|&c| self.lines[2].contains(c))
            .unique()
            .collect();
        let score = ALPHA.find(res[0]);
        return score.unwrap();
    }
}

fn main() {
    println!("Day 03!");

    
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut total1: usize = 0;
    let file_contents = read_lines(path);
    if let Ok(lines) = &file_contents {
        for line in lines {
            let rucksack: Rucksack = Rucksack::from_string(&line);
            let score: usize = rucksack.score();
            total1 += score;
        }
    }
    println!("total = {:?}", total1);

    let mut total2: usize = 0;
    if let Ok(lines) = file_contents {
        for x in &lines.into_iter().chunks(3) {
            let chunk = x.collect::<Vec<String>>();
            let elf_group: ElfGroup = ElfGroup::from_chunk(chunk);
            let score: usize = elf_group.score();
            total2 += score;
        }
    }
    println!("total = {:?}", total2);
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|i| i.unwrap()).collect::<Vec<String>>())
}