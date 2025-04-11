use std::io;

fn main(){
    //taking instance
    let mut str_ing = String::new();
    println!("Enter Your age : ");
    io::stdin().read_line(&mut str_ing).expect("Failed to take input!");
    //taking age
    let age:i32 = str_ing.trim().parse().expect("Failed to assign");
    
    //conditons 
    if age >= 18 && age <=100{
        println!("Eligible for Vote now continue with form...");
        let mut str_ing1 = String::new();
        println!("Enter Your First Name : ");
        io::stdin().read_line(&mut str_ing1).expect("Failed to take input!");
        let fname = str_ing1.trim();
        let mut str_ing2 = String::new();
        println!("Enter Your Last Name : ");
        io::stdin().read_line(&mut str_ing2).expect("Failed to take input!");
        let lname = str_ing2.trim();
        let mut str_ing3 = String::new();
        println!("Enter Your Phone number : ");
        io::stdin().read_line(&mut str_ing3).expect("Failed to take input!");
        let phn:u64 = str_ing3.trim().parse().expect("Wrong input");
        let mut str_ing4 = String::new();
        println!("Enter Your City : ");
        io::stdin().read_line(&mut str_ing4).expect("Failed to take input!");
        let city = str_ing4.trim();
        let mut str_ing5 = String::new();
        println!("Enter Your State : ");
        io::stdin().read_line(&mut str_ing5).expect("Failed to take input!");
        let state = str_ing5.trim();
        //now printing
        println!("You have succesfully submitted voting form. Below are details...");
        println!("Name : {} {} ", fname,lname);
        println!("Age : {} ", age);
        println!("Phone : {} ", phn);
        println!("City : {} ", city);
        println!("State : {} ", state);
        
    }
    else if age < 18 {
        println!("Not eligible for vote due to under 18!");
    }
    else if age > 100{
        println!("Below 100 and above 18 age Can vote only!");
    }
    else{
        println!("Wrong input!");
    }
    
}
