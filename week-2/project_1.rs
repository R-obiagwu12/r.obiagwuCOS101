fn main() {
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 0.1;
    let time: f64 = 5.0;

    let amount = principal * (1.0 + rate).powf(time);
    println!("Amount is {:.2}", amount);
    let compound_interest = amount - principal;

    println!("Compound Interest is ₦{:.2}", compound_interest);
}