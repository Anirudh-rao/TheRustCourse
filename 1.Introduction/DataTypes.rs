fn main(){
     // Signed integer type 
     let x: i32 = -200;
     let y: i32 = 200;
 
     println!("x = {}", x);
     println!("y = {}", y);

    // Unsigned integer type
    let x: u32 = 300;

    println!("x = {}", x);

    // f32 floating point type
    let x: f32 = 3.1;

    // f64 floating point type
    let y: f64 = 45.0000031;

    println!("x = {}", x);
    println!("y = {}", y);

    // boolean type
    let flag1: bool = true;
    let flag2: bool = false;

    println!("flag1 = {}", flag1);
    println!("flag2 = {}", flag2);

    // char type
    let character: char = 'z';

    println!("character = {}", character);

    let special_character: char = '$';
    println!("special_character = {}", special_character);


    //Type Inference in Rust: No need to mention Data types
    let x = 51;

    println!("x = {}", x);
}
