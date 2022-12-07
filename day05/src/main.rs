use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::str;
// use std::rc::Rc;
// use std::iter;
use regex::Regex;
use std::collections::VecDeque;


fn main() {
    println!("Day 05!");

    let re = Regex::new(r"move (\d{1,2}) from (\d{1,2}) to (\d{1,2})").unwrap();

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let file_contents = read_lines(path);
    if let Ok(lines) = &file_contents {
        let mut subs = vec![];
        let mut stacks = vec![];
        let mut moves = vec![];
        let mut in_moves = false;
        for line in lines {
            subs.push(line.as_bytes()
                .chunks(4)
                .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
                .collect::<Vec<&str>>());
        }
        for sub in subs {
            println!("sub = {:?}", sub);
            if in_moves || sub.len() == 0 {
                in_moves = true;
                moves.push(sub.join(""));
            } else {
                stacks.push(sub);
            }
        }
        
        let mut stack_of_stacks: Vec<VecDeque<&str>> = Vec::with_capacity(stacks[0].len());
        for _ in 0..stacks[0].len() {
            stack_of_stacks.push(VecDeque::new());
        }

        for s in stacks {
            for (i, el) in s.iter().enumerate() {
                if !el.to_owned().trim().is_empty() {
                    let order_stack = &mut stack_of_stacks[i];

                    order_stack.push_back(el);
                }
            }
        }

        for (_, m) in moves.iter().enumerate() {
            for cap in re.captures_iter(&m) {
                let from_move_index: usize = cap[2].parse::<usize>().unwrap();
                let to_move_index: usize = cap[3].parse::<usize>().unwrap();
                let count: usize = cap[1].parse::<usize>().unwrap();
                println!("move amount : {} from: {} to: {}", count, from_move_index, to_move_index);

                let mut movers: Vec<&str> = vec![];

                let from_move = &mut stack_of_stacks[from_move_index - 1];
                for _ in 0..count {
                    let item = from_move.pop_front().unwrap();
                    movers.push(item);
                }

                let to_move = &mut stack_of_stacks[to_move_index - 1];
                for mover in movers {
                    to_move.insert(0, mover);
                }
            }
        }

        let answer:String = stack_of_stacks
            .iter()
            .map(|i| 
                i
                .front()
                .unwrap()
                .replace("[", "")
                .replace("]", "")
                .trim()
                .to_string())
            .collect::<Vec<_>>().join("");


        println!("answer = {:?}", answer);
    }
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|i| i.unwrap()).collect::<Vec<String>>())
}
