use std::{
    fs,
    io::{self, BufRead},
};

pub const INPUT: &str = "src/input1.txt";

pub fn solve() {
    let (mut num1, mut num2) = read_nums(INPUT).unwrap_or_else(|e| {
        eprintln!("Error reading numbers from file {}: {}", INPUT, e);
        (Vec::new(), Vec::new())
    });

    num1.sort();
    num2.sort();

    // Part 1
    let mut nums =  Vec::new();
    for i in 0..num1.len() {
        nums.push((num1[i] - num2[i]).abs());
    }
    
    println!("Total distance: {:?}", nums.into_iter().sum::<i32>());

    // Part 2
    let mut sim: i64 = 0;
    let mut j = 0;
    let mut i = 0;

    let max = num1.len();
    loop {
        if j >= max || i >= max {
            break;
        } else if num1[i] < num2[j] {
            i += 1;
        } else if num1[i] > num2[j] {
            j += 1;
        } else if num1[i] == num2[j] {
            let mut t = j;
            while t < max && num1[i] == num2[t]  {
                sim += num1[i] as i64;
                t += 1;
            }
            i += 1;
        }
    }

    println!("Similarity score: {sim}");
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

