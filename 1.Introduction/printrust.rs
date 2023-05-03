fn main(){
    //Single Line Print in Rust
    //Prints in single Line 
    print!("Rust is Fun!");
    print!("I Love Programming");


    //Double Line Print
    //Prints in Two Lines
    println!("Rust is fun!");
    println!("I love Rust programming.");

    //Printing Variables
    let age = 31;
  
    // print the variable using println!
    println!("{}", age);

    // print the variable using print!
    print!("{}", age);

    //Multiple Prints
    let age = 3;
    let Name =  "Jack";

    println!("Name = {}, Age = {}", Name, age);

    //Using PlaceHolders
    let age = 31;
    let name = "Jack";
  
    // print the variable using println!
    println!("Name = {0}, Age = {1}", name, age);

    let age = 31;
    let name = "Jack";
  
    // print the variables using println!
    println!("Name = {name}, Age = {age}");

    //New Line Prints

    print!("Rust is fun!\nI love Rust programming.");

}