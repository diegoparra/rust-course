enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}


fn main() {
    let tickets = vec![
        Ticket::Backstage(100.0, String::from("Diego")),
        Ticket::Vip(200.0, String::from("Carla")),
        Ticket::Standard(100.0)
    ];

    for ticket in tickets{
        match ticket {
            Ticket::Backstage(price, holder) => println!("Price: {:?}, Holder: {:?}", price, holder),
            Ticket::Vip(price, holder) => println!("Price: {:?}, Holder: {:?}", price, holder),
            Ticket::Standard(price) => println!("Price: {:?}", price)
        }
    }
}
