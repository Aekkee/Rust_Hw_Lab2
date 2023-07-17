fn main() { 
    let args: Vec<String> = std::env::args().collect();  
    if args.get(1) == None
    {
        println!("Player 1: N/A");
    } else {
        println!("Player 1: {}", &args[1]);
    }
    if args.get(2) == None
    {
        println!("Player 2: N/A");
    } else {
        println!("Player 2: {}", &args[2]);
    }
}