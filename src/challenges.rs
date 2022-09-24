use std::io;
use rand::prelude::*;
use std::env;
use std::fs;

pub fn challenge_1() {
    println!("----------------Challenge 1----------------");

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* YOUR CODE GOES HERE */
    let average: f64 = (a as f64 + b + c as f64) / 3.00;

    assert_eq! (average, 45.1);
    println! ("Challenge 1 Test passed!");
}

pub fn challenge_2() {
    println!("----------------Challenge 2----------------");

    let temp_c = 23.0;
    let temp_f = celsius_to_fahrenheit(temp_c);

    assert_eq! (temp_f, 73.4);
    println! ("Challenge 2 Test passed!");
}

fn celsius_to_fahrenheit(temp_c: f64) -> f64 {
    (1.8 * temp_c) + 32 as f64
}

pub fn challenge_3() {
    println!("----------------Challenge 3----------------");

    let numbers = [ 1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3 ];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;
    for item in numbers {
        if max < item {
            max = item;
        } else if min > item {
            min = item;
        }

        mean += item as f64;
    }
    mean /= numbers.len() as f64;

    assert_eq! (max, 56);
    assert_eq! (min, -18);
    assert_eq! (mean, 12.5);
    println! ("Challenge 3 Test passed!");
}

pub fn challenge_4() {
    println!("----------------Challenge 4----------------");
    let s = String::from("\t This is a string! ☝     ");
    println!("Input string: {s}");
    let ts = trim_spaces(&s);
    println!("Trimmed string (std): {ts}");
    assert_eq!(ts, s.trim());
    println! ("Challenge 4 Test (std) passed!");
    let tso = trim_spaces_opt(&s);
    println!("Trimmed string (opt): {tso}");
    assert_eq!(tso, s.trim());
    println! ("Challenge 4 Test (opt) passed!");
}

fn trim_spaces(input: &String) -> &str {
    let mut start = 0 as usize;
    let mut end = 0 as usize;
    let mut started = false;
    let bytes = input.as_bytes();
    for (i, &s) in bytes.iter().enumerate() {
        if s == b' ' || s == b'\t' {
            if !started {
                start = i+1;
            }
        } else {
            if !started {
                started = !started;
            }
            end = i;
        }
    }
    &input[start..(end+1)]
}

fn trim_spaces_opt(s: &String) -> &str {
    let mut start = 0 as usize;
    let mut end = 0 as usize;

    for (i, c) in s.chars().enumerate() {
        if c != ' ' && c != '\t' {
            start = i;
            break;
        }
    }

    for (i, c) in s.chars().rev().enumerate() {
        if c != ' ' && c != '\t' {
            end = s.len() - i;
            break;
        }
    }

    &s[start..end]
}

pub fn challenge_5() {
    println!("----------------Challenge 5----------------");

    println!("Guess the number! The number is between 0 and 100.");
    let target_num = thread_rng().gen_range(0..101);
    // println!("Target num: {target_num}");
    loop {
        println!("Input guess:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_e) => println!("Error: {}", _e),
            Ok(_o) => ()
        }

        let guess = match input.trim().parse::<i32>() {
            Err(e) => {
                println!("Parse error: {e}");
                -1
            },
            Ok(i) => i
        };

        if guess == target_num {
            println!("Great, you guessed it, the number was: {guess}");
            break;
        } else if guess >= 0 && guess < target_num {
            println!("No luck. Try a greater number.");
        } else if guess >= 0 && guess > target_num {
            println!("No luck. Try a lower number.");
        }
    }
    println! ("Challenge 5 ended!");
}

pub fn challenge_6() {
    println!("----------------Challenge 6----------------");

    println! ("{} arguments provided", env::args().len());
    if env::args().len() < 2 {
        println! ("Invalid number of arguments!");
        return;
    }

    let file_path = env::args().nth(env::args().len()-2).unwrap();
    println! ("File: {}", file_path);
    let target_name = env::args().nth(env::args().len()-1).unwrap();
    println! ("Name: {}", target_name);

    let file_content = fs::read_to_string(&file_path).unwrap();
    // println! ("File content: \n{}", file_content);

    let mut found = String::new();
    for line in file_content.lines() {
        if line.trim().to_uppercase() == target_name.trim().to_uppercase() {
            found = String::from(line.trim());
            break;
        }
    }

    if found.len() > 0 {
        println! ("The name {} is present in the file {}!", found, file_path);
    } else {
        println! ("The name {} was not found in the file {}!", target_name.trim(), file_path);
    }

    println! ("Challenge 6 ended!");
}