use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter value of pattern: ");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let n: u32 = input.trim().parse().expect("Failed to assign");
    let mut input2 = String::new();
    println!("Please enter pattern choice: ");
    println!("1 - Right Angle Triangle");
    println!("2 - Inverted Pyramid");
    println!("3 - Normal Pyramid");
    println!("4 - Diamond");
    io::stdin().read_line(&mut input2).expect("Failed to read");
    let choice: u32 = input2.trim().parse().expect("Failed to assign"); // Changed u64 to u32

    match choice {
        // Pattern 1: Right-Angled Triangle
        1 => {
            for i in 1..=n {
                for _ in 0..i {
                    print!("* "); 
                }
                println!();
            }
        },

        // Pattern 2: Inverted Pyramid
        2 => {
            for i in (1..=n).rev() {
                for _ in 0..(n - i) {
                    print!(" ");
                }
                for _ in 0..(2 * i - 1) {
                    print!("*");
                }
                println!();
            }
        },

        // Pattern 3: Normal Pyramid
        3 => {
            for i in 1..=n {
                for _ in 0..(n - i) {
                    print!(" "); // Print spaces for alignment
                }
                for _ in 0..(2 * i - 1) {
                    print!("*");
                }
                println!();
            }
        },

        // Pattern 4: Diamond
        4 => {
            // Upper half
            for i in 1..=n {
                for _ in 0..(n - i) {
                    print!(" ");
                }
                for _ in 0..(2 * i - 1) {
                    print!("*");
                }
                println!();
            }

            // Lower half
            for i in (1..n).rev() {
                for _ in 0..(n - i) {
                    print!(" ");
                }
                for _ in 0..(2 * i - 1) {
                    print!("*");
                }
                println!();
            }
        },

        // Default case
        _ => println!("Unknown input! Please enter a valid choice."),
    }
}
