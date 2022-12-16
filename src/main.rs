struct Grocery {
    id: i32,
    quantity: i32,
}

fn display_quantity(item: &Grocery) {
    println!("Item quantity: {}", item.quantity)
}

fn display_id(item: &Grocery) {
    println!("Item ID: {}", item.id)
}

fn main() {
    let coke = Grocery {
        id: 123,
        quantity: 50,
    };

    display_id(&coke);
    display_quantity(&coke);
}

