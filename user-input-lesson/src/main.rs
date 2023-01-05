use std::io;
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_inputs = vec![];
    let mut counter = 0;
    while counter < 2 {
        match get_input() {
            Ok(word) => { 
                all_inputs.push(word);
                counter += 1;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    for word in  all_inputs {
        println!("Original: {:?}", word);
        println!("Transformed to uppercase: {:?}", word.to_uppercase());
    }

}
