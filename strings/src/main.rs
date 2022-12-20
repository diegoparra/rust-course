struct Person {
    name: String,
    age: i32,
    color: String,
}

fn print_name(name: &str) {
    println!("Name: {:?}", name)
}
fn print_color(color: &str) {
    println!("Favourite Color: {:?}", color)
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Diego"),
            age: 31,
            color: "Blue".to_string(),
        },
        Person {
            name: String::from("Carla"),
            age: 39,
            color: "Red".to_string(),
        },
        Person {
            name: String::from("Rosa"),
            age: 70,
            color: "Marraum".to_string(),
        },
    ];

    for p in people {
        if p.age <= 40 {
            print_name(&p.name);
            print_color(&p.color);
        }
    }
}

