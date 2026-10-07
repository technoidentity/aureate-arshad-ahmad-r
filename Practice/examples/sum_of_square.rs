fn main() {
    let n =5;
    let mut sum=0;
    let mut sum_of_square=0;
    
    for i in 1..= n{
       sum +=i;
       sum_of_square  += i*i;
    }
    println!("Sum is: {sum}");
    println!("Sum_of_Square: {sum_of_square}")
}
