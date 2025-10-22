use std::io;

fn main() {
    // Input values for a, b, and c
    println!("Enter the value of a:");
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).unwrap();
    let a: f64 = a_input.trim().parse().unwrap();

    println!("Enter the value of b:");
    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).unwrap();
    let b: f64 = b_input.trim().parse().unwrap();

    println!("Enter the value of c:");
    let mut c_input = String::new();
    io::stdin().read_line(&mut c_input).unwrap();
    let c: f64 = c_input.trim().parse().unwrap();

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots.");
    }
}
