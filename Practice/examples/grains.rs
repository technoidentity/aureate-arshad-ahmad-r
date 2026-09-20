fn main() {
    let square = 5;
    let mut grains = 1;

    for _i in 1..square {
        grains *= 2;
    }

    println!("{grains}");
}
