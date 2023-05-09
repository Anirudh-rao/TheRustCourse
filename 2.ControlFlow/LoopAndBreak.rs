fn main()
{
    let mut number = 0;
    
    //Infinitie Loop Starts Here
    loop{
       number  += 1;
       println!("{}", number);
       
       if number  >= 10{
           //Exit the loop
           break;
       }
    }
}