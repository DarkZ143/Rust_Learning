use std::io::{self, Write};

fn main() {
    let mut arr = [0; 5];

    // Input array elements
    for i in 0..5 {
        let mut input2 = String::new();
        print!("Enter the {} Value => ", i);
        io::stdout().flush().expect("Failed to flush the screen!");
        io::stdin().read_line(&mut input2).expect("Failed to read the input!");
        arr[i] = input2.trim().parse().expect("Enter the Valid values!");
    }
    
    println!("\nYour Entered array is : {:?}\n", arr);

    loop {
        let mut input2 = String::new();
        println!("1 => Largest Element");
        println!("2 => Smallest Element");
        println!("3 => Sorted [ASC] Array");
        println!("4 => Nth element");
        println!("5 => Exit");
        print!("Choose an option => ");
        io::stdout().flush().expect("Failed to flush the screen!");
        
        io::stdin().read_line(&mut input2).expect("Failed to read the input");
        let sel: i32 = input2.trim().parse().expect("Enter a valid number!");

        match sel {
            1 => {
                arr.sort();
                println!("Largest Element in array is : {:?}", arr[arr.len() - 1]);
            },
            2 => {
                arr.sort();
                println!("Smallest Element in array is : {:?}", arr[0]);
            },
            3 => {
                arr.sort();
                println!("Sorted array : {:?}", arr);
            },
            4 => {
                let mut input3 = String::new();
                print!("Enter the Value you want => ");
                io::stdout().flush().expect("Failed to flush the screen output!");
                io::stdin().read_line(&mut input3).expect("Failed to read input!");
                let k: i32 = input3.trim().parse().expect("Enter Valid number!");

                let mut found = false;
                for i in 0..arr.len() {
                    if k == arr[i] {
                        println!("Your value {} is at index ({}) in array...", k, i);
                        found = true;
                    }
                }
                if !found {
                    println!("Your value {} is not present in the array!", k);
                }
            },
            5 => {
                println!("Exiting... Goodbye!");
                break;
            },
            _ => println!("Enter a valid option from 1 to 5..."),
        }

        println!("\nWould you like to perform another action? (y/n)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input!");

        if choice.trim().to_lowercase() != "y" {
            println!("Exiting... Have a great day!");
            break; 
        }
    }
}
