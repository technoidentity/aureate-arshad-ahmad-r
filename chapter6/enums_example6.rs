fn main(){
    let marks: Option<u32> =Some(87);


    match marks{
       Some(100)=> println!("Full Marks"),
       Some(50)=> println!("50 percentage"),
       Some(x)=> println!("Obtained Marks are {x}"),
       None=> println!("Yet to display")

}

}
