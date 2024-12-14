use std::{
    fs,
    io::{self, BufRead},
};

pub const INPUT: &str = "src/advent1.txt";

pub fn solve() {
    let nums = read_nums(INPUT).unwrap_or_else(|e| {
        eprintln!("Error reading numbers from file {}: {}", INPUT, e);
        (Vec::new(), Vec::new())
    });

    // Sort both list
    let (mut num1, mut num2) = nums;
    num1.sort();
    num2.sort();

    let mut nums =  Vec::new();
    for i in 0..num1.len() {
        nums.push((num1[i] - num2[i]).abs());
    }
    println!("Numbers read from the file: {:?}", nums.into_iter().sum::<i32>());
    
}
fn read_nums(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut nums1 = Vec::new();
    let mut nums2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // Stop reading if the line is empty
        if trimmed.is_empty() {
            break;
        }

        let splitter: Vec<_> = trimmed.split_whitespace().collect();
        match splitter[0].parse::<i32>() {
            Ok(num) => nums1.push(num),
            Err(e) => eprintln!("Error parsing line '{}': {}", trimmed, e),
        }
        match splitter[1].parse::<i32>() {
            Ok(num) => nums2.push(num),
            Err(e) => eprintln!("Error parsing line '{}': {}", trimmed, e),
        }
    }

    Ok((nums1, nums2))
}

