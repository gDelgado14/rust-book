use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: args.get(1).unwrap().clone(),
            file_name: args.get(2).unwrap().clone(),
            case_sensitive
        })
    }
}

pub fn search<'a>(query: &str, txt: &'a str) -> Vec<&'a str> {
    let result = txt.lines()
                    .filter(|line| line.contains(query))
                    .collect();

    result
}

pub fn search_case_insensitive<'a>(query: &str, txt: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let result = txt.lines()
                    .filter(|line| line.to_lowercase().contains(&query))
                    .collect();

    result
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.file_name)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensative() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensative() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}