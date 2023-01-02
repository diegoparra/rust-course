use std::collections::HashMap;

fn main() {
    let mut store_items = HashMap::new();

    store_items.insert("Chair", 5);
    store_items.insert("Bed", 3);
    store_items.insert("Table", 2);
    store_items.insert("Couch", 0);
    let mut total_stock = 0;

    for (item, value) in store_items.iter(){
        total_stock += value;

        let stock_count = if value == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", value)
        };

        println!("Item: {:?}, Stock: {:?}", item, stock_count);
    }
    println!("Total of items in stock: {:?}", total_stock);
}
