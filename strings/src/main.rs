struct Person {
    name: String,
    age: i32,
    color: String,
}

fn print_name(name: &Person) {
    println!("Name: {:?}", name.name)
}
fn print_color(color: &Person) {
    println!("Favourite Color: {:?}", color.color)
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
            print_name(&p);
            print_color(&p);
        }
    }
}

