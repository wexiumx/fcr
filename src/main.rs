fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("Total args: {}", args.len());

    for (i, arg) in args.iter().enumerate() {
        println!(" args[{}] = {:?}", i, arg);
    }
}
