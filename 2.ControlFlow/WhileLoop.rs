fn main(){
    //Variable to print multiplication table for
    let i = 2;
    
    //COunter variable stars at 1
    let mut j = 1;
    
    //While Loop for 10 iterations
    while j <= 10{
        let mutlt  = i*j;
        println!("{} * {} =  {}", i,j, mutlt);
        
        j+=1;
    }
}