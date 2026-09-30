fn main() {
    let mut row = 0;

    'outer: loop {
        row += 1;
        let mut column = 0;

        loop {
            column += 1;

            println!("Row: {row}, Column: {column}");

            if row == 2 && column == 2 {
                break 'outer;
            }

            if column == 3 {
                break;
            }
        }
    }

    println!("Loops finished!");
}
