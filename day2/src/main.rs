use std::error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("{}", valid_passwords_count_v1(&buffer));
    println!("{}", valid_passwords_count_v2(&buffer));

    Ok(())
}

fn valid_passwords_count_v1(buffer: &String) -> i32 {
    let passwords = buffer.trim();
    let passwords = passwords.split("\n");

    passwords.fold(0, |acc, line: &str| {
        let splitted_line = line
            .split_terminator(|c| c == '-' || c == ' ')
            .collect::<Vec<&str>>();

        let min = splitted_line[0].parse::<i32>().unwrap();
        let max = splitted_line[1].parse::<i32>().unwrap();
        let letter = splitted_line[2]
            .trim_end_matches(':')
            .parse::<char>()
            .unwrap();
        let password = splitted_line[3];

        let count = password
            .chars()
            .fold(0, |acc, c| if c == letter { acc + 1 } else { acc });

        if max >= count && count >= min {
            acc + 1
        } else {
            acc
        }
    })
}

fn valid_passwords_count_v2(buffer: &String) -> i32 {
    let passwords = buffer.trim();
    let passwords = passwords.split("\n");

    passwords.fold(0, |acc, line: &str| {
        let splitted_line = line
            .split_terminator(|c| c == '-' || c == ' ')
            .collect::<Vec<&str>>();

        let first_char = splitted_line[0].parse::<i32>().unwrap() as usize;
        let second_char = splitted_line[1].parse::<i32>().unwrap() as usize;
        let letter = splitted_line[2]
            .trim_end_matches(':')
            .parse::<char>()
            .unwrap();
        let password = splitted_line[3].to_string();

        if (password.chars().nth(first_char - 1).unwrap() == letter)
            ^ (password.chars().nth(second_char - 1).unwrap() == letter)
        {
            acc + 1
        } else {
            acc
        }
    })
}
