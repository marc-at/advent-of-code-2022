use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use phf::phf_map;

static SCORE_SHAPE: phf::Map<&'static str, i32> = phf_map! {
    "rock" => 1,
    "paper" => 2,
    "scissor" => 3
};

static SCORE_OUTCOME: phf::Map<&'static str, i32> = phf_map! {
    "win" => 6,
    "lose" => 0,
    "draw" => 3,
};

static SHAPE_LOOKUP: phf::Map<&'static str, &'static str> = phf_map! {
    "X" => "rock",
    "Y" => "paper",
    "Z" => "scissor",
};

static SCORE_OUTCOME_2: phf::Map<&'static str, &'static str> = phf_map! {
    "X" => "lose",
    "Y" => "draw",
    "Z" => "win",
};

static MATCH_OUTCOME: phf::Map<&'static str, &'static str> = phf_map! {
    "A X" => "draw",
    "A Y" => "win",
    "A Z" => "lose",
    "B X" => "lose",
    "B Y" => "draw",
    "B Z" => "win",
    "C X" => "win",
    "C Y" => "lose",
    "C Z" => "draw",
};

static MATCH_OUTCOME_PLAY: phf::Map<&'static str, &'static str> = phf_map! {
    "loseA" => "scissor",
    "drawA" => "rock",
    "winA" => "paper",
    "loseB" => "rock",
    "drawB" => "paper",
    "winB" => "scissor",
    "loseC" => "paper",
    "winC" => "rock",
    "drawC" => "scissor",
};

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
                SCORE_SHAPE.get(MATCH_OUTCOME_PLAY.get(play2_key).unwrap()).unwrap() +
                SCORE_OUTCOME.get(play2_outcome).unwrap();
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