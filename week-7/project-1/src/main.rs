use std::io;

// Function to calculate area of trapezium
fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

// Function to calculate area of rhombus
fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// Function to calculate area of parallelogram
fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// Function to calculate area of cube
fn area_cube(side: f64) -> f64 {
    6.0 * side * side
}

// Function to calculate volume of cylinder
fn volume_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("=== Geometry Calculator ===");
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    
    let mut choice = String::new();
    println!("Enter your choice (1-5):");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
    
    match choice {
        1 => {
            // Area of Trapezium
            let mut height_input = String::new();
            println!("Enter height:");
            io::stdin().read_line(&mut height_input).expect("Failed to read input");
            let height: f64 = height_input.trim().parse().expect("Invalid input");
            
            let mut base1_input = String::new();
            println!("Enter base1:");
            io::stdin().read_line(&mut base1_input).expect("Failed to read input");
            let base1: f64 = base1_input.trim().parse().expect("Invalid input");
            
            let mut base2_input = String::new();
            println!("Enter base2:");
            io::stdin().read_line(&mut base2_input).expect("Failed to read input");
            let base2: f64 = base2_input.trim().parse().expect("Invalid input");
            
            let result = area_trapezium(height, base1, base2);
            println!("Area of Trapezium = {:.2}", result);
        },
        2 => {
            // Area of Rhombus
            let mut diag1_input = String::new();
            println!("Enter diagonal1:");
            io::stdin().read_line(&mut diag1_input).expect("Failed to read input");
            let diag1: f64 = diag1_input.trim().parse().expect("Invalid input");
            
            let mut diag2_input = String::new();
            println!("Enter diagonal2:");
            io::stdin().read_line(&mut diag2_input).expect("Failed to read input");
            let diag2: f64 = diag2_input.trim().parse().expect("Invalid input");
            
            let result = area_rhombus(diag1, diag2);
            println!("Area of Rhombus = {:.2}", result);
        },
        3 => {
            // Area of Parallelogram
            let mut base_input = String::new();
            println!("Enter base:");
            io::stdin().read_line(&mut base_input).expect("Failed to read input");
            let base: f64 = base_input.trim().parse().expect("Invalid input");
            
            let mut altitude_input = String::new();
            println!("Enter altitude:");
            io::stdin().read_line(&mut altitude_input).expect("Failed to read input");
            let altitude: f64 = altitude_input.trim().parse().expect("Invalid input");
            
            let result = area_parallelogram(base, altitude);
            println!("Area of Parallelogram = {:.2}", result);
        },
        4 => {
            // Area of Cube
            let mut side_input = String::new();
            println!("Enter side length:");
            io::stdin().read_line(&mut side_input).expect("Failed to read input");
            let side: f64 = side_input.trim().parse().expect("Invalid input");
            
            let result = area_cube(side);
            println!("Area of Cube = {:.2}", result);
        },
        5 => {
            // Volume of Cylinder
            let mut radius_input = String::new();
            println!("Enter radius:");
            io::stdin().read_line(&mut radius_input).expect("Failed to read input");
            let radius: f64 = radius_input.trim().parse().expect("Invalid input");
            
            let mut height_input = String::new();
            println!("Enter height:");
            io::stdin().read_line(&mut height_input).expect("Failed to read input");
            let height: f64 = height_input.trim().parse().expect("Invalid input");
            
            let result = volume_cylinder(radius, height);
            println!("Volume of Cylinder = {:.2}", result);
        },
        _ => {
            println!("Invalid choice! Please select between 1-5.");
        }
    }
}