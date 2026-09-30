 
enum Command {
    Add(String),
    Remove(usize),
    List,

}

impl Command{
    fn describe(&self){
        match self{
            Command::Add(item) => println!("Add {item} to shopping list"),
            Command::Remove(index) =>println!("Remove item at index {index}"),
            Command::List => println!("Show all shopping items"),
            
        }
    }
}

fn main(){
    let first=Command::Add(String::from("Milk"));
    let second=Command::Remove(2);
    let third=Command::List;

    first.describe();
    second.describe();
    third.describe();

}




