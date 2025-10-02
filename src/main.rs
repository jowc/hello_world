const TOUCHDOWN_POINTS: i32 = 6;

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
    };

    let _season: &'static str = "Rainy";
    let point_score: i32 = 35 + TOUCHDOWN_POINTS;

    #[allow(unused_variables)]
    let event_time: &'static str = "6:00 PM";
    let event_time: i32 = 1900;
    

    println!("Print my name {name} using curly brackets, residing in {location}, with a score of {point_score} at {event_time}");
}
