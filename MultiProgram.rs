use std::io;
fn main(){
    let mut opt = String::new();
    println!("Enter Your Choice : ");
    println!("1 => Reverse the number.");
    println!("2 => Prime Number.");
    println!("3 => Factorial.");
    println!("4 => Armstrong number. ");
    println!("5 => Digit Count.");
    println!("6 => Palindrome.");
    io::stdin().read_line(&mut opt).expect("Failed to read!");
    let _choice:u64= opt.trim().parse().expect("Failed to parse integer!");
    let mut input = String::new();
    println!("Enter the number : ");
    io::stdin().read_line(&mut input).expect("Failed to take input");
    let mut n:u64 = input.trim().parse().expect("Failed to parse the input!");
    let mut rev = 0;
    let mut temp;
    match _choice {
        //First case for reverse number
        1 => {
            while n > 0{
                temp = n%10;
                rev = rev * 10 + temp;
                n/=10;
            }
            println!("Reverse number is : {} ", rev);
        },
        //Second case for Prime Number
        2 => {
            for i in 2..=n/2+1{
                if n%i==0{
                    println!("Number is not prime!");
                    break;
                }
                else{
                    println!("It is prime Number.");
                    break;
                }
            }
            
        },
        //Third case for Factorial
        3 =>{
            if n==0{
                println!("{}",1);
                
            }
            else{
                let mut result = 1;
                for i in 1..=n {
                result *= i;
                }
            println!("Factorial of {} is {}", n, result);
            }
            
        },
        //fourth for Armstrong
        4 => {
            let c = n;
            let mut _sum = 0;
            while n > 0{
                temp = n%10;
                _sum += temp.pow(3);
                n/=10;
            }
            if c == _sum{
                println!("Yes {} is armstrong", c);
            }
            else {
                println!("No {} is not armstrong", c);
            }
            
        },
        //Digit count
        5 => {
            let _c = n;
            let mut count = 0;
            if n == 0{
                println!("Number has 1 digit ");
            }
            else{
                while n > 0{
                    n = n/10;
                    count += 1;
                }
            println!("Number {} has {} digits", _c, count);}
        },
        //Palindrome number
        6 => {
            let _c = n;
            while n > 0{
                temp = n%10;
                rev = rev * 10 + temp;
                n /= 10;
            }
            if _c == rev {
                println!("{} is palindrome.", _c);
            }
            else{
                println!("{} is not palindrome.",_c);
            }
        }
        _ => println!("Invalid input!"),
    }
}
