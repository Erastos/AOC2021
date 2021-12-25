use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Submarine {
    distance: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn process_directions(&mut self, contents: &Vec<String>) {
        for action in contents {
            let mut action = action.split(" ");
            let verb = action.next().unwrap();
            let num: i32 = action.next().unwrap().parse().unwrap();
            // println!("{}: {}", verb, num);
            if verb == "forward" {
                self.distance += num;
                self.depth += self.aim * num;
            } else if verb == "up" {
                self.aim -= num;
            } else if verb == "down" {
                self.aim += num;
            } else {
                panic!("Verb not found");
            }
        }
    }
}

fn get_file_contents(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    lines
}

fn main() {
    let mut sub = Submarine {
        distance: 0,
        depth: 0,
        aim: 0,
    };

    let file_name: String = env::args().nth(1).unwrap();
    let contents = get_file_contents(&file_name);
    sub.process_directions(&contents);
    println!("{:?}", sub);
    println!("Result: {}", sub.depth * sub.distance);
}
