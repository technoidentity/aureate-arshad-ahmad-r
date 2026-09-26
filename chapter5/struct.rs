 struct Book{
      title:String,
      author:String,
      pages:u32,
}

fn main(){
     let book1= Book{
          title:String::from("Rich Dad and Poor Dad"),
          author:String::from("Robert Kiyosaki"),
          pages: 336,

     };
     let book2= Book{
          title:String::from("The Psychology of Money"),
          author:String::from("Morgan Housel"),
          pages:256,
     };
    println!("Title of book1 is {}",book1.title);
    println!("Title of book2 is {}",book2.title);
}






