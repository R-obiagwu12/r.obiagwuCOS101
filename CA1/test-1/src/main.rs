// Electricity bill installer

use std::io;

// Input values
fn main() {
 let name = String::new();
 println!("Enter Customer name");  
 io::stdin().read_line(&mut name).expect("Failed to read input");
 let name = name.trim();

 let mut units_consumed = String::new();
 println!("Enter Units consumed");
 io::stdin().read_line(&mut units_consumed).expect("Failed to read input");
 let units_consumed:f64 = units_consumed.trim().parse().expect("Failed to read input");

 let rate:f64 = 
 if {units_consumed >= 0.0 && units_consumed <= 100.0;
           "20.0"
  }else if {units_consumed >= 101.0 && units_consumed <= 300.0;
           "35.0"
  }else if condition {units_consumed >= 301.0;
           "50.0"
  }else {
      Invalid input
  };
   let total = rate * units_consumed as f64;
   let final_amount = if units_consumed > 500.0{
    total + 500.0
   }else{
    total
   };
  

   // Output results
   println!("Customers Name is {}",name);
   println!("Units consumed is {}",units_consumed);
   println!("Rate is {}",rate);
   println!("Total amount is {}",total);
   println!("Final amount",final_amount);
 
} 
