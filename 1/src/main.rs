use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_nums_from_file(file_name: &str) -> Vec<u32> {
    let input_file = File::open(file_name).unwrap();
    BufReader::new(input_file).lines().map(|x| x.unwrap().parse::<u32>().unwrap()).collect()
}

fn get_num_of_increases(contents: &Vec<u32>) -> u32 {
    let mut increases: u32 = 0;
    for i in 0..contents.len()-1 {
        if contents[i] < contents[i+1] {
            increases += 1;
        };
    };
    increases
}

fn get_nums_of_sum_increases(contents: &Vec<u32>) -> u32 {
    let mut increases: u32 = 0;
    let mut first_sum = contents[0] + contents[1] + contents[2];
    for i in 0..contents.len()-2 {
        let current_sum = contents[i] + contents[i+1] + contents[i+2];
        if first_sum < current_sum {
            increases += 1;
        };
        first_sum = current_sum;
    };
    increases
}

fn main() {
    let file_name = &env::args().nth(1).unwrap();
    let contents = get_nums_from_file(&file_name);
    println!("{:?}", get_nums_of_sum_increases(&contents));
}
