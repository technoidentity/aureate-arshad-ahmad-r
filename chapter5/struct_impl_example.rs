struct Book{
    title:String,
    author:String,
    pages:u32,
}

impl Book{
    fn print(&self){
        println!("Title is {}, author: {}, pages: {}",self.title,self.author, self.pages );
        
        
}}

fn main(){
    let user1=Book{
         title:String::from("Rich Dad Poor Dad"),
         author:String::from("Robert kiyosaki"),
         pages:336
    };
    let user2=Book{
        title:String::from("The physology of Money"),
        author:String::from("Morgan housel"),
        pages:255,
    };
    user1.print();
    user2.print();
}



