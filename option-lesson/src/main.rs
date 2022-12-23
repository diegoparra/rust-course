struct Student {
    name: String,
    locker: Option<i32>,
}


fn main() {
    let new_student = Student{
        name: "Diego Parra".to_owned(),
        locker: Some(123765),
    };

    println!("User Name: {:?}", new_student.name);

    match new_student.locker {
        Some(locker) => println!("Locker ID: {:?}", locker),
        None => println!("Locker ID not found"),
    }

}
