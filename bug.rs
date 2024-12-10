fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let ptr = vec.as_ptr();
    // Drop vec here, memory is deallocated
    drop(vec);
    println!("{:p}", ptr); //Dangling pointer. Dereferencing it is UB.
    //Reading from this dangling pointer is undefined behavior.
}