fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x = vec![5, 6, 7, 8, 9, 10, 11];
    
    let min_len = std::cmp::min(v.len(), x.len());
    
    for index in 0..min_len {
        let sum = v[index] + x[index];
        println!("{} + {} = {}", v[index], x[index], sum);
    }
}
