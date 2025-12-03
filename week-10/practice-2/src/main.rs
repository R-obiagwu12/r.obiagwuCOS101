fn main() {
    let v = vec![10,20,30];
    //vector v owns th eobject in heap

    let v2=v;   // moves ownership to v

    display(v2.clone());
    //v2 is moved to display and v2 is initialized

    println!("In main {:?}",v2);


}

fn display(v:Vec<i32>){
    println!("Inside display:{:?}",v);
}

