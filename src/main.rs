#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

/// Convert a number between two bases.
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    let mut value = 0;
    for (i, digit) in number.iter().rev().enumerate() {
        if digit >= &from_base {
            return Err(Error::InvalidDigit(*digit));
        }
        value += digit * from_base.pow(i as u32);
    }

    let mut converted = vec![];
    while value > 0 {
        converted.push(value % to_base);
        value /= to_base;
    }
    converted.reverse();

    if converted.is_empty() {
        converted.push(0);
    }

    Ok(converted)
}

fn main() {
    // Test cases for the `convert` function
    let test_cases = vec![
        (vec![4, 2], 10, 2),      // Convert 42 (decimal) to binary
        (vec![1, 0, 1, 0], 2, 10), // Convert 1010 (binary) to decimal
        (vec![2, 5], 10, 16),     // Convert 25 (decimal) to hexadecimal
        (vec![], 10, 2),          // Convert empty input (0) to binary
        (vec![0], 10, 2),         // Convert 0 (decimal) to binary
    ];

    for (number, from_base, to_base) in test_cases {
        match convert(&number, from_base, to_base) {
            Ok(result) => {
                println!(
                    "Converted {:?} from base {} to base {}: {:?}",
                    number, from_base, to_base, result
                );
            }
            Err(e) => {
                println!(
                    "Failed to convert {:?} from base {} to base {}: {:?}",
                    number, from_base, to_base, e
                );
            }
        }
    }
}
