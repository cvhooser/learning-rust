use std::{env, error::Error, fs};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("Incorrect number of arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    let case_sensitive = !env::var("CASE_SENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.filename)?;

  for line in search(&config.query, &contents, config.case_sensitive) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
  let mut res = Vec::new();

  let query = if !case_sensitive {
    query.to_lowercase()
  } else {
    String::from(query)
  };
  for line in contents.lines() {
    let line_to_test = if !case_sensitive {
      line.to_lowercase()
    } else {
      String::from(line)
    };
    if line_to_test.contains(&query) {
      res.push(line);
    }
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents, true)
    )
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false))
  }
}
