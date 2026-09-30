enum PaymentMethod{
    Cash,
    UPI,
    Card
}

struct Payment{
    amount: u32,
    method:PaymentMethod,
}
fn main(){
    let payment =Payment{
        amount:500,
        method:PaymentMethod::UPI,
    };
    
    let payment2 =Payment{
        amount:100,
        method:PaymentMethod::Card,
    };

    println!("Amount is {}",payment.amount);
    match payment.method{
        PaymentMethod::Cash =>println!("Cash Payment"),
        PaymentMethod::Card=>println!("Card Payment"),
        PaymentMethod::UPI=> println!("UPI Payment"),
         
    }
    println!("Amount: {}",payment2.amount);
    match payment2.method{
        PaymentMethod::Cash => println!("Cash Payment"),
        PaymentMethod::Card => println!("Card Payment"),
        PaymentMethod::UPI => println!("UPI Payment"),
    }
        
    
    
}
