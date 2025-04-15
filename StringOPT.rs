use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let vowels = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

    loop {
        let mut input = String::new();
        print!("Enter a string => ");
        io::stdout().flush().expect("Failed to flush the screen!");
        io::stdin().read_line(&mut input).expect("Failed to read the line!");
        
        let string = input.trim().to_string(); // Storing input safely

        println!("\nChoose an operation:");
        println!("1 => Check for Vowel");
        println!("2 => Check for Alphabet");
        println!("3 => Find ASCII");
        println!("4 => Length of String");
        println!("5 => Check Palindrome");
        println!("6 => Remove non-alphabet characters");
        println!("7 => Remove Spaces");
        println!("8 => Sum of numbers present in string");
        println!("9 => Calculate frequency of characters");
        println!("10 => Exit");
        

        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read the line!");
        
        let choice: i32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue; // Restart loop
            }
        };

        match choice {
            1 => {
                if !string.is_empty() && string.chars().any(|c| vowels.contains(&c)) {
                    println!("Yes, vowel present in your input.");
                } else {
                    println!("No vowel found.");
                }
            },
            2 => {
                if !string.is_empty() && string.chars().all(|c| c.is_alphabetic()) {
                    println!("Yes, all characters in the string are alphabets.");
                } else {
                    println!("No, the string contains non-alphabet characters.");
                }
            },
            3 => {
                println!("ASCII Values of characters:");
                for c in string.chars() {
                    println!("{} => {}", c, c as u8);
                }
            },
            4 => {
                let mut count = 0;
                for _ in string.chars() {
                    count += 1;
                }
                println!("Length of the string => {}", count);
            },
            5 => {
                let rev: String = string.chars().rev().collect();
                if string == rev {
                    println!("Yes, '{}' is a palindrome.", string);
                } else {
                    println!("No, '{}' is not a palindrome.", string);
                }
            },
            6 => {
                let removed: String = string.chars().filter(|c| c.is_alphabetic()).collect();
                println!("Filtered String (Only Alphabets): {}", removed);
            },
            7 => {
                let filtered: String = string.chars().filter(|c| !c.is_whitespace()).collect();
                println!("String without spaces: {}", filtered);
            },
            8 => {
                let sum: u32 = string.chars()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_digit(10).unwrap())
                    .sum();

                println!("Sum of numbers present in string => {}", sum);
            },
            9 => {
                let mut frequency: HashMap<char, usize> = HashMap::new();
                for c in string.chars() {
                    *frequency.entry(c).or_insert(0) += 1;
                }
                println!("Character Frequency Count:");
                for (c, count) in frequency.iter() {
                    println!("'{}' => {}", c, count);
                }
            },
            10 => {
                println!("Exiting program...");
                break;
            },
            _ => println!("Enter a valid input!")
        }

        // Asking if the user wants to continue
        let mut again = String::new();
        print!("\nDo you want to perform another action? (Y/N) => ");
        io::stdout().flush().expect("Failed to flush the screen!");
        io::stdin().read_line(&mut again).expect("Failed to read the line!");

        let again = again.trim().to_lowercase();
        if again != "y" {
            println!("Goodbye!");
            break;
        }
    }
}
