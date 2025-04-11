use std::io;

fn main(){
    //taking instance
    let mut x = String::new();
    println!("Enter the first number : ");
    io::stdin().read_line(&mut x).expect("Failed to read input");
    //first variable assign
    let n: i32 = x.trim().parse().expect("Failed to assign number!");
    //taking another instance
    let mut y = String::new();
    println!("Enter the Second number : ");
    io::stdin().read_line(&mut y).expect("Failed to read input");
    //second variable assign
    let m: i32 = y.trim().parse().expect("Failed to assign number!");
    let mut choose = String::new();
    println!("Choose the Operation +, -, *, /, %");
    io::stdin().read_line(&mut choose).expect("Failed to read input");
    let user_selection = choose.trim();
    
    //rasing conditions for choices
    if user_selection == "+"{
        println!("Addition is : {}", n+m);
    }
    else if user_selection == "-"{
        println!("Substraction is : {}", n-m);
    }
    else if user_selection == "*"{
        println!("Multiply is : {}", n*m);
    }
    else if user_selection == "/"{
        println!("Divisor is : {}", n/m);
    }
    else if user_selection == "%"{
        println!("Modulo is : {} ", n%m);
    }
    else{
        println!("Wrong Operation! and exiting....");
    }
    
}
