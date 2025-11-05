//Campus Cafe Order System
 
use std::io;
    
fn main() {
    loop{
    println!("Campus Cafe Order System");
    println!("Code        Item              Price (₦)");
    println!("T           Tea               800");
    println!("C           Coffee            1200");
    println!("S           Sandwich          1500");
    println!("J           Juice             2000");

    // Input item code
    let mut code = String::new();
    println!("Enter item code (T/C/S/J):");
    io::stdin().read_line(&mut code).expect("Failed to read input");
    let code = code.trim().to_uppercase();

    // Input quantity
    let mut qty = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty: i32 = qty.trim().parse().expect("Failed to read input");

    // Match item price
    let price = match code.as_str() {
        "T" => 800.0,
        "C" => 1200.0,
        "S" => 1500.0,
        "J" => 2000.0,
        _ => {
            println!("Invalid code entered!");
            return;
        }
    };

    // Calculate total and discount
    let total = price * qty as f64;
    let final_amount = if total > 500000.0 {
        total * 0.95   // Apply 5% discount
    } else {
        total
    };

    // Output result
    println!("\nItem Code: {}", code);
    println!("Quantity: {}", qty);
    println!("Total Cost: ₦{:.2}", total);
    if total > 5000.0 {
        println!("Discount Applied: 5%");
    }
    println!("Final Amount Payable: ₦{:.2}", final_amount);
    
    println!("If you want to continue, type 'y' and if you want to exit the program, type 'n'");
    let mut u = String::new();
    io::stdin().read_line(&mut u).expect("Failed input");
    let u = u.trim();
    if u == "y" {continue;}
    else if u == "n" {break;}
    else{println!("Invalid input");
    break;}
    }
 }   





