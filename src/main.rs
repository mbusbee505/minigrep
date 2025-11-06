use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut flags: Vec<String> = Vec::new();
    let mut inputs: Vec<String> = Vec::new();

    for arg in &args[1..] {
        if arg.starts_with("-") {
            flags.push(arg.clone());
        } else {
            inputs.push(arg.clone());
        }
    }
    
    let config = Config::build(&flags, &inputs).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1); 
    }
}

fn run(config: Config) -> Result <(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(flags: &[String], inputs: &[String]) -> Result<Config, &'static str> {
        if inputs.len() < 2 {
            return Err("not enough arguments");
        }
        let query = inputs[0].clone();
        let file_path = inputs[1].clone();

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for flag in flags {
            if flag == "-i" {
                ignore_case = true;
            }
        }

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}