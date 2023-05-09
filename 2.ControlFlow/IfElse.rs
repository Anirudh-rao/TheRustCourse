fn main(){
    let x =  7;
    
    let condition  =  x > 5;
    
    println!("condition is  {}", condition);
    
    let number = 10;

    // condition to check if number is greater than zero
    if number > 0 {
        println!("{} is greater than 0", number);
    }

    println!("End of program");
    
    //If-Else
    let number2 = -2;

    // condition to check if number is greater than zero
    if number2 > 0 {
        println!("{} is greater than 0", number2);
    } else {
        println!("{} is less than or equal to 0", number2);
    }
    
    //if Else if
    
    let number3 = -2;

    if number > 0 {
        println!("{} is positive", number3);
    } else if number < 0 {
        println!("{} is negative", number3);
    } else {
        println!("{} is equal to 0", number3);
    }
}