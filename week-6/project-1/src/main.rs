use std::io;

fn main() {
    // Display menu
    println!("=== RESTAURANT MENU ===");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    // Get food choice
    let mut food_choice = String::new();
    println!("Enter food code (P/F/A/E/W): ");
    io::stdin().read_line(&mut food_choice).expect("Failed to read input");
    let food_choice = food_choice.trim();
    
    // Get quantity
    let mut quantity = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: i32 = quantity.trim().parse().expect("Please enter a valid number");

    // Determine price based on food choice
    let price = match food_choice {
        "P" | "p" => 3200,
        "F" | "f" => 3000,
        "A" | "a" => 2500,
        "E" | "e" => 2000,
        "W" | "w" => 2500,
        _ => {
            println!("Invalid food choice!");
            return;
        }
    };

    // Calculate total charges
    let mut total_charges = price * quantity;

    // Apply 5% discount if total is greater than N10,000
    if total_charges > 10000 {
        let discount = (total_charges as f32) * 0.05;
        total_charges = total_charges - discount as i32;
        println!("5% discount applied! Discount amount: N{}", discount);
    }

    // Display order summary
    println!("\n=== ORDER SUMMARY ===");
    println!("Food Item: {}", get_food_name(food_choice));
    println!("Price per item: N{}", price);
    println!("Quantity: {}", quantity);
    println!("Total Charges: N{}", total_charges);
}

fn get_food_name(code: &str) -> String {
    match code {
        "P" | "p" => "Poundo Yam/Edinkaiko Soup".to_string(),
        "F" | "f" => "Fried Rice & Chicken".to_string(),
        "A" | "a" => "Amala & Ewedu Soup".to_string(),
        "E" | "e" => "Eba & Egusi Soup".to_string(),
        "W" | "w" => "White Rice & Stew".to_string(),
        _ => "Unknown".to_string()
    }
}
