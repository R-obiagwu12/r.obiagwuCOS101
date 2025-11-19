use std::io;

fn main() {
    let mut name = String::new();
    println!("Enter student's name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    // Input 3 test scores
    let mut s1 = String::new();
     println!("Enter score 1:");
     io::stdin().read_line(&mut s1).expect("Failed to read input");
     let s1: f64 = s1.trim().parse().expect("Failed to read input");
     if s1 > 100.0 || s1 < 0.0 {println!("Invalid input")};

    let mut s2 = String::new();
     println!("Enter score 2:");
     io::stdin().read_line(&mut s2).expect("Failed to read input");
     let s2: f64 = s2.trim().parse().expect("Failed to read input");
    if s2 > 100.0 || s2 < 0.0 {println!("Invalid input")};

    let mut s3 = String::new();
    println!("Enter score 3:");
    io::stdin().read_line(&mut s3).expect("Failed to read input");
    let s3: f64 = s3.trim().parse().expect("Failed to read input");
    if s3 > 100.0 || s3 < 0.0 {println!("Invalid input")};

    // Calculate average
    let average:f64 = (s1 + s2 + s3) / 3.0;

    // Determine grade
    let mut grade = "";
    if average >= 70.0 && average <= 100.0 {
        grade = "A";
    } else if average >= 60.0 && average < 70.0 {
        grade = "B";
    } else if average >= 50.0 && average < 60.0 {
        grade = "C";
    } else if average >= 45.0 && average < 50.0 {
        grade = "D";
    } else {
        grade = "F";
    };
    if average > 100.0 || average < 0.0
    {println!("Value calculated is erroneous and false")};

    // Print result
    println!("\nStudent Name: {}", name);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);
}
