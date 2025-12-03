fn main() {
    let v =vec![101,250,330,400];
    //vector v stores the object in heap


    //Only a sinfgle variable owns the heap memory at a given time
    let v2 = v;
    //Here 2 variables own the heap value
    //2 pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory acess,so it detects a race condition
    //as 2 variables point in the same heap

    println!("{:?}",v2);
   
}

