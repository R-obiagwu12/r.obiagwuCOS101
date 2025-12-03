fn main() {
    //a list of nos
    let v = vec![15,20,35,30,35,40];
    print_vector(v.clone());
    println!("{}",v[0]);

}
fn print_vector(x:Vec<i32>){
    println!("Inside print_vector function {:?}",x);
}

