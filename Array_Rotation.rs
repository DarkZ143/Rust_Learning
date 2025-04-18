use std::io::{self, Write};

fn left_rotation(arr: &mut Vec<usize>, n: usize) {
    if arr.is_empty() { return; }

    let len = arr.len();
    let shift = n % len;

    for _ in 0..shift {
        let first = arr[0];
        for i in 0..len-1 {
            arr[i] = arr[i+1];
        }
        arr[len - 1] = first;
    }
}

fn right_rotation(arr: &mut Vec<usize>, n: usize) {
    if arr.is_empty() { return; }

    let len = arr.len();
    let shift = n % len;

    for _ in 0..shift {
        let last = arr[len - 1];
        for i in (1..len).rev() {
            arr[i] = arr[i-1];
        }
        arr[0] = last;
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter array size => ");
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin().read_line(&mut input).expect("Failed to read input!");
        let n: usize = input.trim().parse().expect("Invalid number!");

        let mut arr: Vec<usize> = Vec::new();
        for i in 0..n {
            let mut value = String::new();
            print!("Enter index {} value => ", i);
            io::stdout().flush().expect("Flush failed!");
            io::stdin().read_line(&mut value).expect("Failed to read input!");
            let element: usize = value.trim().parse().expect("Invalid number!");
            arr.push(element);
        }

        println!("Original array => {:?}", arr);

        let mut rotate_value = String::new();
        print!("Enter rotation count => ");
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin().read_line(&mut rotate_value).expect("Failed to read input!");
        let k: usize = rotate_value.trim().parse().expect("Invalid number!");

        let mut choice = String::new();
        print!("Enter rotation type (1 for Left, 2 for Right) => ");
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin().read_line(&mut choice).expect("Failed to read input!");
        let rotation_type: usize = choice.trim().parse().expect("Invalid choice!");

        match rotation_type {
            1 => {
                left_rotation(&mut arr, k);
                println!("After Left Rotation by {} positions => {:?}", k, arr);
            },
            2 => {
                right_rotation(&mut arr, k);
                println!("After Right Rotation by {} positions => {:?}", k, arr);
            },
            _ => println!("Wrong choice! Enter 1 for Left or 2 for Right."),
        }

        let mut again = String::new();
        print!("Do you want to perform another operation? (Y/N) => ");
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin().read_line(&mut again).expect("Failed to read input!");
        let again = again.trim().to_uppercase();

        if again != "Y" {
            println!("Exiting the program.");
            break;
        }
    }
}
