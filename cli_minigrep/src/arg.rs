// Arg is an object that can extract a string or bool value from itself. 
pub trait Arg {
    type Error;
    fn get_string(&self) -> Result<String, Self::Error>;
    fn get_bool(&self) -> Result<bool, Self::Error>;
}

impl Arg for String {
    type Error = String;

    // --some-flag=hello -> Result<"hello">
    fn get_string(&self) -> Result<String, Self::Error> {
        let parts: Vec<&str> = self.split("=").collect();
        if parts.len() < 2 {
            return Ok("".to_string())
        }
        Ok(parts[1].to_string())
    }

    // --some-flag -> Result<true>
    // --some-flag=true -> Result<true>
    // --some-flag=false -> Result<false>
    // --some-flag=not-a-boolean -> Result<Self::Error>
    fn get_bool(&self) -> Result<bool, Self::Error> {
        match self.get_string() {
            Ok(s) => {
                match s.parse() {
                    Ok(b) => Ok(b),
                    Err(_) => Err(format!("argument {} should be 'true' or 'false'", self)),
                }
            },
            Err(_) => Ok(true),
        }
    }
}
