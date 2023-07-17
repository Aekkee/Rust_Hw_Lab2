fn main() { 
    let args: Vec<String> = std::env::args().collect();  
    let x_arg = if args.len() < 2 { "" } else { &args[1] };  
    let x: f32 = x_arg.parse().unwrap_or(0.0);  
    let y = 3.1416 * x * x;
    println!("Cirlcle area: {}",y); 
}