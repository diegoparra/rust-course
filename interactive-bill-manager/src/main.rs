// User stories:

// L1: I want to add bills, including the name and the amount owed.

struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill)
    }
}

fn main_menu() {}

fn main() {
    println!("Hello, world!");
}
