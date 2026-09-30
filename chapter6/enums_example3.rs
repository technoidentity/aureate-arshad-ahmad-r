enum OrderStatus{
     Pending,
     Shipped,
     Delivered,
}

fn main(){
   let status = OrderStatus::Shipped;


   match status{
       OrderStatus::Pending => println!("Your order is being prepared"),
       OrderStatus::Shipped => println!("Your order is on the way"),
       OrderStatus::Delivered => println!("Your order is Delivered"),
}
}

