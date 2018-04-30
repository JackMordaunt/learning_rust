use std::env;
use arg::Arg;

pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("not enough arguments".to_string());
        }
        let mut case_sensitive = !env::var("MATCH_CASE").is_err();        
        let query = args[1].clone();
        let path = args[2].clone();
        let flags = args.iter().filter(|arg| arg.contains("-"));
        for f in flags {
            if f.contains("match") && f.contains("case") {
                case_sensitive = f.get_bool()?;
            }
        }
        Ok(Config{
            query,
            path,
            case_sensitive,
        })
    }
}