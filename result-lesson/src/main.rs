struct Customer {
    age: i32,
}

fn validate_customer_age(customer: &Customer) ->  Result<(), String> {
    if customer.age < 21 {
        Err("Customer is under 21".to_owned())
    } else {
        Ok(())
    }
    
}


fn main() {
    let diego: Customer = Customer { age: 20 };

    let purchase = validate_customer_age(&diego);

    println!("{:?}", purchase);
}
