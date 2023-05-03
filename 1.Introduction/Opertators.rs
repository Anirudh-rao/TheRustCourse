fn main() {
    //Arthematic Operator

    let a = 20;
    let b = 2;

    // add two variables using + operator
    let mut x = a + b;
    println!("{} + {} = {}", a, b, x);

    // subtract two variables using - operator
    let mut y = a - b;
    println!("{} - {} = {}", a, b, y);

    // multiply two variables using * operator
    let z = a * b;
    println!("{} * {} = {}", a, b, z);

    let dividend = 21;
    let divisor = 8;

    // arithmetic division using / operator with integers
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    let dividend = 21.0;
    let divisor = 8.0;

    // arithmetic division using / operator with floating point values
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    let dividend = 21;
    let divisor = 8;

    // arithmetic remainder using % operator
    let remainder = dividend % divisor;
  
    println!("{} % {} = {}", dividend, divisor, remainder);

    //Assigment Operators
    let mut x = 1;

    // compound assignment operators
    x += 3;

    let mut a = 2;
  
    // arithmetic addition and assignment
    a += 3;

    println!("a = {}", a);

    //comparison operators
    let mut a = 7;
    let mut b = 3;
    
    // use of comparison operators
    let mut c = a > b;
    let mut d = a < b;
    let mut e = a == b;
    
    println!("{} >= {} is {}", a, b, c);
    println!("{} <= {} is {}", a, b, d);
    println!("{} == {} is {}", a, b, e);

    //Logical Operators
    let mut a = true;
    let mut b = false;
    
    // logical AND operation
    let mut c = a && b;

    // logical OR operation
    let mut d = a || b;

    // logical NOT operation
    let mut e = !a;
    
    println!("{} && {} = {}", a, b, c);
    println!("{} || {} = {}", a, b, d);
    println!("!{} = {}", a, e);
}