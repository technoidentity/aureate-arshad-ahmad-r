enum Payment_Method{
      UPI,
      Cash,
      Credit_Card,
}

fn main(){
    let Method = Payment_Method::Cash;
   

    match Method{
       Payment_Method::Cash => println!("Accept the Cash Payment"),
       Payment_Method::UPI => println!("Scan the QR Code"),
       Payment_Method::Credit_Card => println!("Insert the Card"),
}
}
