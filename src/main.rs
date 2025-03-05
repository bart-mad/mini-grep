use std::env;
use std::fs;

fn main(){
    // Read the function args and parse them to variables
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::new(&args);

    // Read the target file
    let file_content: String = fs::read_to_string(&config.file_path)
        .expect("Failed to read the file");

    println!("In file: {}",config.file_path);
    println!("{}",file_content);

}
struct Config {
    query: String,
    file_path: String,
}

impl Config{
    fn new(args: &[String]) -> Config{
    
        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        if args.len() > 3{
            // ToDo! - print to stderr
            pritnln!("Ignoring extra arguments passed (>2)");
        }

        Config{query: args[1].clone(),
               file_path: args[2].clone()} 

    }

    }
