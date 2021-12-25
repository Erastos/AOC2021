use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_contents(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    lines
}

fn determine_most_common_bit(nth: usize, nums: &Vec<&[u8]>) -> u32 {
    let mut num_ones: usize = 0;
    for num in nums {
        if num[nth] == b'1' {
            num_ones += 1;
        }
    }
    if num_ones >= nums.len() - num_ones {
        1
    } else {
        0
    }
}

fn collect_most_common_bits(nums: &Vec<&[u8]>) -> Vec<u32> {
    let str_len = nums[0].len();
    let mut bits: Vec<u32> = Vec::<u32>::new();
    for i in 0..str_len {
        bits.push(determine_most_common_bit(i, nums));
    };
    bits
}

fn collect_most_common_bits_filter<'a>(nums: &'a Vec<&[u8]>) -> Vec<&'a [u8]> {
    let str_len = nums[0].len();
    let mut vec = Vec::new();
    for i in 0..str_len {
        vec = nums.into_iter().filter(|x| x[i] == (std::char::from_digit(0, 10).unwrap() as u8)).collect::<Vec<&&[u8]>>();
    };
    let vec = vec.iter().map(|x| **x).collect();
    vec
}

fn create_number(vec: &Vec<u32>) -> u32 {
    vec.iter().fold(0, |a, x| {
        let a = a << 1;
        let a = a + x;
        a
    })
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let contents = get_file_contents(&file_name);
    let contents: Vec<&[u8]> = contents.iter().map(|x| x.as_bytes()).collect();
    let nums = collect_most_common_bits_filter(&contents);
    println!("{:?}", nums);
}
