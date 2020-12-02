use std::error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let entries = get_input(buffer);

    if let Some((x, y)) = sums_to_target(entries, 2020) {
        println!("{:?}", x * y);
    } else {
        println!("No matching numbers found")
    }
    Ok(())
}

fn get_input(input: String) -> Vec<i32> {
    input
        .trim()
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

// Solution taken from https://web.stanford.edu/class/cs9/sample_probs/TwoSum.pdf
fn sums_to_target(entries: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let mut entries = entries.clone();
    entries.sort();

    let mut res: Option<(i32, i32)> = None;

    for (i, x) in entries.iter().enumerate() {
        let sibling = target - x;

        if let Ok(sibling_index) = entries.binary_search(&sibling) {
            if sibling_index != i {
                res = Some((*x, entries[sibling_index]));
            }

            if i > 0 && entries[i - 1] == *x {
                res = Some((*x, entries[i - 1]));
            }

            if i < entries.len() - 1 && entries[i + 1] == *x {
                res = Some((*x, entries[i + 1]));
            }
        }
    }

    res
}
