fn main() {
    let mut data = vec![1, 2, 3];
    let first = data[0].clone(); // Clone to avoid holding borrow
    data.push(4);         // Mutable borrow is now safe
    println!("{}", first); 
}
