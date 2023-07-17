//To run this enter value like this: cargo run --bin (CtoF/FtoC) Value
//Ex. cargo run --bin FtoC 10
fn main() {
    let args: Vec<String> = std::env::args().collect();  
    if &args[1] == " "
    {
        println!("re-enter");
    } if &args[1] == "CtoF"{
        let y: f32 = args[2].parse().unwrap_or(0.0);  
        println!("C to F: {}", 1.8 * y + 32.0);
    } else if &args[1] == "FtoC"{
        let y: f32 = args[2].parse().unwrap_or(0.0);  
        println!("F to C: {}", 5.0/9.0 * (y - 32.0));
    } 
}