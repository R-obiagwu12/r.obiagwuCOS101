use std::io;

fn main() {
    // Input experience
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Faied to read input");
    let experienced = experience_input.trim().parse().expect("Faied to read input");

    // Input age
    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Faied to read input");
    let age: u32 = age_input.trim().parse().expect("Faied to read input");

    // Determine incentive
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000 * 12 // Monthly incentive converted to annual
        } else {
            0 // No incentive specified for age 28–29
        }
    } else {
        100_000
    };

    println!("Annual incentive: ₦{}", incentive);
}

