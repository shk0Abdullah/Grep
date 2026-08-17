

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    println!("{:?}", args[1]);

    let file_path = args[2].clone();
    println!("File path: {}", file_path);
    let contents = std::fs::read_to_string(file_path)?;
    println!("{:#?}", contents);

    Ok(())
    
    // let file = File::open(args[1]).clone().unwrap();
    // let reader = BufReader::new(file);
    // for line in reader.lines() {
    //     println!("{}", line.unwrap());
    // }
}