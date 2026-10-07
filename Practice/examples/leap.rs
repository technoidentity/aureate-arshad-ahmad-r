fn main() {
    let year = 2024;

    if year % 400 == 0 {
        println!("true");
    } else if year % 4 == 0 && year % 100 != 0 {
        println!("true");
    } else {
        println!("false");
    }
}
