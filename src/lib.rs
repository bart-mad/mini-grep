use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the target file
    let file_content: String = fs::read_to_string(&config.file_path)?;

    let contents: Vec<&str> = if config.ignore_case {
        case_insensitive_search(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for item in contents {
        println!("{}", item);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skipping the 0th arg - file name

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("No querry string passed"),
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path passed"),
        };

        if args.next() != None {
            eprintln!("Too many arguments passed - ignoring extra arguments");
        }

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_hit_test() {
        let query: &str = "Apple";
        let contents: &str = "Banana is yellow
Apple is red
Pear is green
Bear is hungry";

        assert_eq!(vec!["Apple is red"], search(query, contents));
    }

    #[test]
    fn case_sensitive_test() {
        let query: &str = "RUST";
        let contents: &str = "I like trains,
but RUST is cool,
rust is very cool";

        assert_eq!(vec!["but RUST is cool,"], search(query, contents));
    }

    #[test]
    fn case_insensitive_test() {
        let query: &str = "RUST";
        let contents: &str = "I like trains,
but RuSt is cool,
rust is very cool";

        assert_eq!(
            vec!["but RuSt is cool,", "rust is very cool"],
            case_insensitive_search(query, contents)
        );
    }
}
