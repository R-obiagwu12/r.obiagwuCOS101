// Student loan estimator repayment

use std::io; 

fn main() {
    loop {

    let mut p = String::new();
 println!("Enter Principal(P)");  
 io::stdin().read_line(&mut p).expect("Failed to read input");
 let p:f64 = p.trim().parse().expect("Failed to read input");

   let mut r = String::new();
 println!("Enter Rate(R)");  
 io::stdin().read_line(&mut r).expect("Failed to read input");
 let r:f64 = r.trim().parse().expect("Failed to read input");

 let mut t = String::new();
 println!("Enter Time(T)");  
 io::stdin().read_line(&mut t).expect("Failed to read input");
 let t:f64 = t.trim().parse().expect("Failed to read input");

 let a:f64 = p * (1.0 + r/100.0).powf(t);
 println!("Amount is {:.2}",a);

 let ci:f64 = a - p;
 println!("Compound interest is {:.2}",ci);

    }
 }   



  

