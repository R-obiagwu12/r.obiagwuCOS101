fn main() {
    
    // Using Vec::New()
    let v : Vec<i64> = Vec::new();

    // printing the size of the vector
    println!("\nThe length of Vec::new is {}",v.len());

    // Using macro
    let v = vec!["Ronald", "Bruce", "Chigo", "Kamsi",];

    // printing the size of the vector
    println!("\nThe length of the vec macro is {}",v.len());

}
    