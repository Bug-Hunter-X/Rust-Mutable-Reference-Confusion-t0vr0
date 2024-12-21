fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x
    *y = 10; // Modify x through y
    println!("x = {}", x); // prints x = 10
    println!("z = {}", *z); // prints z = 10

    let mut a = 5;
    let b = &mut a;
    let c = &mut a; // Multiple mutable references are allowed 
    *b = 10;
    *c = 15;
    println!("a = {}", a); // prints a = 15
}