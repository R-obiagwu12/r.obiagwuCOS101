use std::io;

fn main() {
        println!("\nCompound Interest Calculator");

        // Input P, R, T
        let mut p = String::new();
        let mut r = String::new();
        let mut t = String::new();

        println!("Enter Principal (P):");
        io::stdin().read_line(&mut p).expect("Failed to read input");
        println!("Enter Rate (R in %):");
        io::stdin().read_line(&mut r).expect("Failed to read input");
        println!("Enter Time (T in years):");
        io::stdin().read_line(&mut t).expect("Failed to read input");

        // Convert inputs to numbers
        let p: f64 = p.trim().parse().expect("Failed to read input");
        let r: f64 = r.trim().parse().expect("Failed to read input");
        let t: f64 = t.trim().parse().expect("Failed to read input");

        // Calculate total amount and compound interest
        let a = p * (1.0 + r / 100.0).powf(t);
        let ci = a - p;

        // Display results
        println!("Total Amount (A): {:.2}", a);
        println!("Compound Interest: {:.2}", ci);


    
}

