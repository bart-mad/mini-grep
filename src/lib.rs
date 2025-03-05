use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read the target file
    let file_content: String = fs::read_to_string(&config.file_path)?;

        println!("{}",file_content);

        Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
    
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
