fn main() {
    let mut number = 0;

    loop {
        number += 1;

        if number == 3 {
            continue;
        }

        if number == 6 {
            break;
        }

        println!("Number: {number}");
    }
}
