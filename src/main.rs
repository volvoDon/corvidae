use std::{env, error::Error, fs};


fn main() {
    let config : Vec<String> = env::args().collect();
    let config = Config::new(&config).unwrap();
    println!("key: {}",config.key);
    println!("file: {}",config.file);
    println!("argument: {}",config.argument);

    let mut content = readfile(&config.file).unwrap();
    scramble(&mut content, &config).unwrap();


}

struct Config {
    key: String,
    file: String,
    argument: String,
}

impl Config {
    fn new (args : &Vec<String>) -> Result<Config,&'static str> {
        if args.len() < 4 {
            return Err("not enough arguments")
        }
        
        let key = args[1].clone();
        let file = args[2].clone();
        let argument = args[3].clone();
        Ok(Config{key,file,argument})
    }    
}

fn readfile(file:&String) -> Result<String,Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)

}

fn scramble(content: &mut String, config: &Config) -> Result<(),&'static str> {
    if config.argument == "F" {
        return Err("TRIGGERED BY TYPE F")
    }
    println!("{}",content);
    Ok(())
}
