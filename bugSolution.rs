fn main() {
    let mut x = 5;
    { // This creates a new scope
        let y = &mut x; 
        *y = 10; 
    } // The borrow of y ends here
    let z = &mut x; 
    *z = 12;
}