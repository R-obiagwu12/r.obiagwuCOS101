use std::io;
    
fn main() {
    loop{
    println!("Simple Inventory Discount System");
    println!("Code        Item              Price (₦)");
    println!("L           Laptop            550000");
    println!("M           Monitor           120000");
    println!("K           Keyboard          15000");
    println!("H           Headset           25000");

    // Input item code
    let mut code = String::new();
    println!("Enter item code (L/M/K/H):");
    io::stdin().read_line(&mut code).expect("Failed to read input");
    let code = code.trim().to_uppercase();

    // Input quantity
    let mut qty = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty: i32 = qty.trim().parse().expect("Failed to read input");

    // Match item price
    let price = match code.as_str() {
        "L" => 550000.0,
        "M" => 120000.0,
        "K" => 15000.0,
        "H" => 25000.0,
        _ => {
            println!("Invalid code entered!");
            return;
        }
    };

    // Calculate total and discount
    let total = price * qty as f64;
    let final_amount = if total > 500000.0 {
        total * 0.93   // Apply 7% discount
    } else {
        total
    };

    // Output result
    println!("\nItem Code: {}", code);
    println!("Quantity: {}", qty);
    println!("Total Cost: ₦{:.2}", total);
    if total > 500000.0 {
        println!("Discount Applied: 7%");
    }
    println!("Final Amount Payable: ₦{:.2}", final_amount);
    
    println!("If you want to continue, type 'y' and if you want to quit the program, type 'n'");
    let mut u = String::new();
    io::stdin().read_line(&mut u).expect("Failed input");
    let u = u.trim();
    if u == "y" {continue;}
    else if u == "n" {break;}
    else{println!("Invalid input");
    break;}
    }
 }   


