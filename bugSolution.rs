fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    //Avoid creating a dangling pointer by using a reference or cloning the data before dropping the vector.
    let safe_vec = vec.clone();
    drop(vec);
    println!("{:?}", safe_vec);
}