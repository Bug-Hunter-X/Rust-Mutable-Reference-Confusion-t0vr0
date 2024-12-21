fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    } // y goes out of scope here
    println!("x = {}", x); // prints x = 10

    let mut a = 5;
    let mut b = &mut a; // Only one mutable reference at a time.    
    *b = 10; 
    println!("a = {}", a); // prints a = 10

    //Another option to avoid multiple mutable references: Using a closure
    let mut c = 5; 
    { 
      let mut d = &mut c;
      *d = 12;
    }
    println!("c = {}", c); // Prints c = 12
} 