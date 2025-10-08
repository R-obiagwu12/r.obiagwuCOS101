fn main() {
    let t: f64 = 450_000.0;
    let m: f64 = 1_500_000.0;
    let h: f64 = 750_000.0;
    let d: f64 = 2_850_000.0;
    let a: f64 = 250_000.0;

    //average
    let s = t + m + h + d + a;
    println!("Sum is {:.3}", s);
    let a = s/5.0;
    println!("Average is {:.3}", a);
}