use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;
use lazy_static::lazy_static;
// use ::phf::{phf_map, Map};

// static SCORE_SHAPE: phf::Map<&'static str, i32> = phf_map! {
//     "rock" => 1,
//     "paper" => 2,
//     "scissor" => 3
// };

lazy_static! {
    static ref SCORE_SHAPE: HashMap<&'static str, i32> = {
        let mut map = HashMap::new();
        map.insert("rock", 1);
        map.insert("paper", 2);
        map.insert("scissor", 3);
        map
    };

    static ref SCORE_OUTCOME: HashMap<&'static str, i32> = {
        let mut map = HashMap::new();
        map.insert("win", 6);
        map.insert("lose", 0);
        map.insert("draw", 3);
        map
    };
    
    static ref SHAPE_LOOKUP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("X", "rock");
        map.insert("Y", "paper");
        map.insert("Z", "scissor");
        map
    };

    static ref SCORE_OUTCOME_2: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("X", "lose");
        map.insert("Y", "draw");
        map.insert("Z", "win");
        map
    };
    
    static ref MATCH_OUTCOME: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("A X", "draw");
        map.insert("A Y", "win");
        map.insert("A Z", "lose");
        map.insert("B X", "lose");
        map.insert("B Y", "draw");
        map.insert("B Z", "win");
        map.insert("C X", "win");
        map.insert("C Y", "lose");
        map.insert("C Z", "draw");
        map
    };

    static ref MATCH_OUTCOME_PLAY: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("loseA", "scissor");
        map.insert("drawA", "rock");
        map.insert("winA", "paper");
        map.insert("loseB", "rock");
        map.insert("drawB", "paper");
        map.insert("winB", "scissor");
        map.insert("loseC", "paper");
        map.insert("winC", "rock");
        map.insert("drawC", "scissor");
        map
    };
}

fn main() {
    println!("Day 02!");
    
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    if let Ok(lines) = read_lines(path) {
        let mut score: i32 = 0;
        let mut score_2: i32 = 0;
        for line in lines {
            let line_str: &str = &line.unwrap().parse::<String>().unwrap();
            let line_arr: Vec<&str> = line_str.split(" ").collect();

            score += 
                SCORE_SHAPE.get(SHAPE_LOOKUP.get(line_arr[1]).unwrap()).unwrap() + 
                SCORE_OUTCOME.get(MATCH_OUTCOME.get(line_str).unwrap()).unwrap();

            let play2_outcome: &str = &SCORE_OUTCOME_2.get(line_arr[1]).unwrap();
            let play2_key: &str = &(play2_outcome.to_owned() + line_arr[0]);
            score_2 += 
                SCORE_OUTCOME.get(play2_outcome).unwrap() + 
                *SCORE_SHAPE.get(MATCH_OUTCOME_PLAY.get(play2_key).unwrap()).unwrap();
        }
        println!("-----------------------------");
        println!("score = {:?}", score);
        println!("score_2 = {:?}", score_2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// -----------------------------
// score = 14163
// score_2 = 12091