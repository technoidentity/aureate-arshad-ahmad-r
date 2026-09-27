enum Command{
    Add(String),
    List,
}


fn main(){
    let command=Command::Add(String::from("Milk"));

    // When want only one pattern we use if let and skip other part

    if let Command::Add(item)=command{
        println!("Add {item} to list")
    }
}
