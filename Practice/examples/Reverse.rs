fn main() {
    let numbers =[10,20,30,40,50];
    let mut a =Vec::new();
    for i in numbers.iter().rev(){
        a.push(*i);}
        println!("{:?}",a)
    
}
