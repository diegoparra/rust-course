fn main() {
    let my_numbers = vec![10, 20, 30, 40];

    println!("Number of elements {:?}", my_numbers.len());

    for number in my_numbers {
        if number == 30 {
            println!("Thirty");
        } else {
            println!("{number}");
        }
    }
}
