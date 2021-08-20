use std::{env, error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    println!("search result:");
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(a) => a,
            None => return Err("please supply query string"),
        };
        let filename = match args.next() {
            Some(a) => a,
            None => return Err("please supply filename"),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.
        ";

    #[test]
    fn one_result() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, &CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "Rust";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, CONTENTS)
        )
    }
}
