use std::fs::File;
use std::io::Write;

fn main() {
    // Create and write to file
    let mut file = File::create("nigerian_breweries_drinks.txt").expect("create failed");
    
    // Write headers
    file.write_all("Nigerian Breweries Plc - Drink Categories\n".as_bytes()).expect("write failed");
    file.write_all("===========================================\n\n".as_bytes()).expect("write failed");
    
    // Write table header
    file.write_all("+-----------------+-----------------+-------------------+\n".as_bytes()).expect("write failed");
    file.write_all("| Lager           | Stout           | Non-Alcoholic     |\n".as_bytes()).expect("write failed");
    file.write_all("+-----------------+-----------------+-------------------+\n".as_bytes()).expect("write failed");
    
    // Write each row manually
    file.write_all("| 33 Export       | Legend          | Maltina           |\n".as_bytes()).expect("write failed");
    file.write_all("| Desperados      | Turbo King      | Amstel Malta      |\n".as_bytes()).expect("write failed");
    file.write_all("| Goldberg        | Williams        | Malta Gold        |\n".as_bytes()).expect("write failed");
    file.write_all("| Gulder          |                 | Fayrouz           |\n".as_bytes()).expect("write failed");
    file.write_all("| Heineken        |                 |                   |\n".as_bytes()).expect("write failed");
    file.write_all("| Star            |                 |                   |\n".as_bytes()).expect("write failed");
    
    // Write table footer
    file.write_all("+-----------------+-----------------+-------------------+\n".as_bytes()).expect("write failed");
    
    println!("Drink categories saved to 'nigerian_breweries_drinks.txt' in table format!");
}