use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main(){
    // Read the function args and parse them to variables
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search for: {}",config.query);
    println!("In file: {}",config.file_path);

    if let Err(e) = run(config){
        println!("Application error  {e}");
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read the target file
    let file_content: String = fs::read_to_string(&config.file_path)?;

        println!("{}",file_content);

        Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str>{
    
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        if args.len() > 3{
            // ToDo! - print warning (to stderr?) and continue
            println!("Ignoring extra arguments passed (>2)");
        }

        Ok(Config{query: args[1].clone(),
               file_path: args[2].clone()})

    }

    }
