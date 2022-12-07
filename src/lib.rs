pub use std::env;
use std::error::Error;
use std::fs;
pub fn run (config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let res = if config.shift_both {
        search(&config.query, &contents)
    } else {
        search_shift_both(&config.query, &contents)
    };
    for line in res {
        println!("{}", line);
    }
    Ok(())
}
pub struct Config {
    query: String,
    filename: String,
    shift_both: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough aegs");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let shift_both = env::var("CASE_INSENSITIVE").unwrap() == "1";
        Ok(Config { query, filename, shift_both })
    }
}
pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}
pub fn search_shift_both<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one () {
        let query = "Naxida";
        let contents = "Good moning, Sumeru' people.
My name is Naxida, practicing be a nice god has two year and half.
I like singing, jumping and playing basketball.productive.
I like naxida.";
        assert_eq!(vec!["My name is Naxida, practicing be a nice god has two year and half."], search(query, contents));
    }
    #[test]
    fn two() {
        let query = "Naxida";
        let contents = "Good moning, Sumeru' people.
My name is Naxida, practicing be a nice god has two year and half.
I like singing, jumping and playing basketball.productive.
I like naxida.";
        assert_eq!(vec!["My name is Naxida, practicing be a nice god has two year and half.",
         "I like naxida."], search_shift_both(query, contents));

    }
}