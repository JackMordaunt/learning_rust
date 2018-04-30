use std::env;
use arg::Arg;

pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn from_args<I>(args: I) -> Result<Config, String> 
        where I: IntoIterator<Item = String>,
    {
        let mut args = args.into_iter();
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("no query provided".to_string()),
        };
        let path = match args.next() {
            Some(path) => path,
            None => return Err("no path provided".to_string()),
        };

        let mut case_sensitive = !env::var("MATCH_CASE").is_err();     
        let flags = args.filter(|arg| arg.contains("-"));
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