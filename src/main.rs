use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Query: {}",query);
    println!("Filer path: {}",file_path);
    
}