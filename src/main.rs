fn main() {
    println!("Hello, world!");

    println!("My name is Joseph Chikeme");

    print!("Currently learning Rust");

    print!(" Using print vs println \n");

    let name = "Joseph Chikeme";
    let mut location = "Abuja";
    
    // new location
    if location.len() > 0 {
        location = "Vilnius";
    }

    println!("Print my name {name} using curly brackets, residing in {location}");
}
