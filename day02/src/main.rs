use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;



fn main() {
    println!("Day 02!");

    let score_shape: HashMap<&str,i32> = HashMap::from([
        ("rock", 1),
        ("paper", 2),
        ("scissor", 3)
    ]);

    let score_outcome: HashMap<&str,i32> = HashMap::from([
        ("win", 6),
        ("lose", 0),
        ("draw", 3)
    ]);
    
    let shape_lookup: HashMap<&str,&str> = HashMap::from([
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissor")
    ]);

    let score_outcome_2: HashMap<&str,&str> = HashMap::from([
        ("X", "lose"),
        ("Y", "draw"),
        ("Z", "win")
    ]);
    
    let matchup_outcome: HashMap<&str,&str> = HashMap::from([
        ("A X", "draw"),
        ("A Y", "win"),
        ("A Z", "lose"),
        ("B X", "lose"),
        ("B Y", "draw"),
        ("B Z", "win"),
        ("C X", "win"),
        ("C Y", "lose"),
        ("C Z", "draw")
    ]);

    let matchup_outcome_play: HashMap<&str,&str> = HashMap::from([
        ("loseA", "scissor"),
        ("drawA", "rock"),
        ("winA", "paper"),
        ("loseB", "rock"),
        ("drawB", "paper"),
        ("winB", "scissor"),
        ("loseC", "paper"),
        ("winC", "rock"),
        ("drawC", "scissor")
    ]);
    
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    if let Ok(lines) = read_lines(path) {
        let mut score: i32 = 0;
        let mut score_2: i32 = 0;
        for line in lines {
            let line_str: &str = &line.unwrap().parse::<String>().unwrap();
            let line_arr: Vec<&str> = line_str.split(" ").collect();

            score += 
                score_shape.get(shape_lookup.get(line_arr[1]).unwrap()).unwrap() + 
                score_outcome.get(matchup_outcome.get(line_str).unwrap()).unwrap();

            let play2_outcome: &str = &score_outcome_2.get(line_arr[1]).unwrap();
            let play2_key: &str = &(play2_outcome.to_owned() + line_arr[0]);
            score_2 += 
                score_outcome.get(play2_outcome).unwrap() + 
                *score_shape.get(matchup_outcome_play.get(play2_key).unwrap()).unwrap();
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