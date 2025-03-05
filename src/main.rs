use std::env;
use std::fs;

fn main(){
    // Read the function args and parse them to variables
    let args: Vec<String> = env::args().collect();
    
    let (query, file_path) = parse_config(&args);

    // Read the target file
    let file_content: String = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    println!("In file: {}",file_path);
    println!("{}",file_content);

}

fn parse_config(args: &[String]) -> (&str, &str){

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    (query, file_path)

}