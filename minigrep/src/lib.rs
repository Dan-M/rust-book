use std::{error::Error, fs};

pub struct Args {
    query: String,
    file_path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Args { query, file_path })
    }
}

#[derive(Debug)]
pub struct MatchedLine<'a> {
    line: &'a str,
    line_no: u32,
}

impl<'a> MatchedLine<'a> {
    fn new(line: &str, line_no: u32) -> MatchedLine {
        MatchedLine { line, line_no }
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in:\n {}", args.query, args.file_path);
    let contents = fs::read_to_string(args.file_path)?;
    for line in search(&args.query, &contents) {
        println!("{} - {}", line.line_no, line.line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<MatchedLine<'a>> {
    let mut result = Vec::new();
    let mut current_line_no = 0;
    for line in contents.lines() {
        current_line_no += 1;
        if line.contains(query) {
            result.push(MatchedLine::new(line, current_line_no));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // multi line string using \
        let contents = "\
Rust:
Save, fast, productive.
Pick three";
        assert_eq!("Save, fast, productive.", search(query, contents)[0].line);
    }
}
