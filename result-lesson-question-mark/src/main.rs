enum JobKind {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Assembly,
}

enum Status {
    Active,
    Terminated
}

struct Employees {
    title: JobKind,
    active: Status,
} 

fn is_allowed(guy: &Employees) -> Result<(), String> {
    match guy.active {
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => (),
        
    }

    match guy.title {
       JobKind::Maintenance =>  Ok(()),
       JobKind::Marketing =>  Ok(()),
       JobKind::Manager =>  Ok(()),
        _ => Err("Employee not allowed to enter into the build".to_owned())
    }
}

fn print_access(employee: &Employees) -> Result<(), String> {
    let try_access = is_allowed(employee)?;
    println!("Access granted");
    Ok(())
    
}

fn main() {
    let diego = Employees{
        title: JobKind::Manager,
        active: Status::Terminated,
    };

   match print_access(&diego) {
        Err(e) => println!("Access Denied: {:?}", e),
        _ => ()
    }
}
