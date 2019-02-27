
pub fn run() {
    let name = "Tyrese";
    let mut age = 19; 
    
    println!("My name is {} and I a am {}", name, age);

    age = 21; 
    println!("My name is {} and I am {}", name, age);
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let ( my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}