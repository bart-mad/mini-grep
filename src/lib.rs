use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read the target file
    let file_content: String = fs::read_to_string(&config.file_path)?;

    let contents: Vec<&str> = if config.ignore_case{
        case_insensitive_search(&config.query,&file_content)
    }
    else{
        search(&config.query, &file_content)
    };


    for item in contents{
        println!("{}",item);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
    
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        if args.len() > 3{
            // ToDo! - print warning (to stderr?) and continue
            eprintln!("Ignoring extra arguments passed (>2)");
        }

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query: args[1].clone(),
               file_path: args[2].clone(),
                ignore_case})

    }

    }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines(){

        if line.contains(query) {

            results.push(line);

        }
    }

    results
}

pub fn case_insensitive_search<'a>(query: &str,contents: &'a str) -> Vec<&'a str>{

    let query: String = query.to_lowercase();

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines(){

        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_hit_test(){
        let query: &str = "Apple";
        let contents:  &str = "Banana is yellow
Apple is red
Pear is green
Bear is hungry";

        assert_eq!(vec!["Apple is red"], search(query, contents));

    }

    #[test]
    fn case_sensitive_test(){
        let query: &str = "RUST";
        let contents: &str = "I like trains,
but RUST is cool,
rust is very cool";

        assert_eq!(vec!["but RUST is cool,"],search(query,contents));
    }

    #[test]
    fn case_insensitive_test(){
        let query: &str = "RUST";
        let contents: &str ="I like trains,
but RuSt is cool,
rust is very cool";

        assert_eq!(vec!["but RuSt is cool,","rust is very cool"],
                    case_insensitive_search(query,contents));
    }
}

