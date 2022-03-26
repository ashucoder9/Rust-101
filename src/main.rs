fn main() {
    println!("Hello, world!");
    println!("This is a simple program to demonstrate how programs run in Rust language. Kudos to all the learners.");
    println!("Hail Rust\n");

    // Calling a simple function
    variables();

    let a = 33;
    let b = 36;
    add(a, b);
}

// Declaring a function without arguments
fn variables() {
    // Declaring Various Data Types
    let a = 5;                      // integer (i32, i64 are most common)
    let b = 9.0;                    // floating point number
    let c = true;                   // boolean
    let d = "Ashutosh Tripathi";    // string
    
    // Declaring Array
    let array = [1,2,3,4,5,6,7,8,9,10];

    // Declaring Tuple and initializing
    let tuple = (100, false, "Tony");
    let (x, y, z) = tuple;

    println!("The value of new variable is {}", x);

    // Calling a function with arguments
    greet(d);
}


// Declaring a function with arguments
// You have to use '&' if you use dynamic sizing of parameters
fn greet(name: &str) {
    println!("Hello {}", name);
}


fn add(x: i32, y:i32) {
    println!("{}", x+y);
}
