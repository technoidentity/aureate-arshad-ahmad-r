 
enum Command {
    Add(String),
    Remove(usize),
    List,
}

fn main(){
    let command=Command::Add(String::from("Add Milk"));


   match command{
        Command::Add(item) => println!("{item} to shopping list"),
        Command::Remove(index) => println!("Remove the item at index {index}"),
        Command::List =>println!("Show all products in list"),
}
}






