// Rust program to get student names and grades

use std::io;

fn main() {

    // input name
    println!("\nPlease Enter your name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is: {}", name);

    // input scores
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nEnter your first score.");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: i32 = input1.trim().parse().expect("Input not an integer");
    if a > 100{
        println!("Invalid number");
    }

     println!("\nEnter your second score.");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: i32 = input2.trim().parse().expect("Input not an integer");
    if b > 100{
        println!("Invalid number");
    }

     println!("\nEnter your third score.");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: i32 = input3.trim().parse().expect("Input not an integer");
    if c > 100{
        println!("Invalid number");
     }   

    let avg = (a + b + c) / 3;
    println!("Average of your score is {}",avg);

    //grades
    let _grades : char =
    if avg >= 70 && avg <= 100 {
        'A'
    }else if avg >= 60 && avg < 70  {
        'B'
    }else if avg >= 50 && avg < 60 {
        'C'
    }else if avg >= 45 && avg < 50 {
        'D'
    }else {
        'F'
    };   


        


    }

