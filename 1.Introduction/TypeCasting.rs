fn main(){
    // assign a floating point f64 value to decimal variable
    let decimal: f32 = 64.31;

     // convert decimal variable to u16 integer type using as keyword
    let integer = decimal as u16;
 
    println!("decimal = {}", decimal);
    println!("integer = {}", integer);

    //Char to INT
    let character: char = 'A';

    // convert char type to u8 integer type
    let integer = character as u8;

    println!("character = {}", character);
    println!("integer = {}", integer);

    // only u8 integer data type can be converted into char
    let integer: u8 = 65;
  
    // convert integer to char using the as keyword
    let character = integer as char;

    println!("integer = {}" , integer);
    println!("character = {}", character);


    /*
    let integer: i32 = 65;
  
    // convert integer to char using the as keyword
    let character = integer as char;

    println!("integer = {}" , integer);
    println!("character = {}", character);
    
    ERROR We need to use U8 only to convert
    */

    //Boolean Conversion
    let boolean1: bool = false;
    let boolean2: bool = true;
  
    // convert boolean type to integer
    let integer1 = boolean1 as i32;
    let integer2 = boolean2 as i32;

    println!("boolean1 = {}", boolean1);
    println!("boolean1 = {}", boolean2);
    println!("integer1 = {}", integer1);
    println!("integer2 = {}", integer2);

    /*
    let decimal: f32 = 65.321;
  
    // convert float to char data type
    let character = decimal as char;

    println!("decimal = {}", decimal);
    println!("character = {}", character);
    
    ERROR : Cannot Convert Float to Char
    */
}