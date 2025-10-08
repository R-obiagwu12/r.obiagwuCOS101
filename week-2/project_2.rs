fn main() {
    let principal:f64 = 510_000.0;
    let rate:f64 = 0.05;
    let time:f64 = 3.0;


    let amount:f64 = principal * (1.0 - rate).powf(time);

    let final_value:f64 = amount - principal;

    println!("The final Price of The TV is {:.2}", final_value);
}