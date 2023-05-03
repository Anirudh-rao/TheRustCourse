fn main() {
    // variable to store integer value
    let age = 31;
    println!("Age: {}", age);

    // variable to store floating-point value
    let salary = 342523.23;
    println!("Salary: {}", salary);

    // variable to store string
    let name = "Jackie";
    println!("Name: {}", name);

    //Change Value of a Variable
    //Declare a variable with value 1
    let x =1;
    println!("x = {}", x);

    //Change value to 2
    /*
    x = 2;
    println!("x = {}", x);
    
    "The Above Code will throw error
    as By Default Rust Variables are immutable : cannot be changed"
    */

    //Mutability in Rust
    let mut x = 1;
    println!("The Value of x = {}",x);
    //Allows are x is mutable
    x = 2;
    println!("x = {}", x);


    // declare a constant
    const PI:f32 = 3.14;
    println!("Initial Value of PI: {}", PI);

    // change value of PI
    /*
    PI = 535.23;
    println!("Update Value of PI: {}", PI);
    "The Above Code will throw error
    as Constants cannot be changed 
    */
}