use std::error::Error;
use std::fs;

pub struct Config {
    pub key: String,
    pub file: String,
    pub argument: String,
}

impl Config {
    pub fn new (args : &Vec<String>) -> Result<Config,&'static str> {
        if args.len() < 4 {
            return Err("not enough arguments")
        }
        
        let key = args[1].clone();
        let file = args[2].clone();
        let argument = args[3].clone();
        Ok(Config{key,file,argument})
    }    
}

pub fn readfile(file:&String) -> Result<String,Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)

}

pub fn scramble(content: &String, config: &Config) -> Result<(),&'static str> {
    if config.argument == "F" {
        return Err("TRIGGERED BY TYPE F")
    }
    println!("not enc: {}",content);
    let b_content =  content.as_bytes().clone();
    let mut nb_content= vec![];
    for item in b_content {
        nb_content.push(item+1)
    }
    let content = String::from_utf8(nb_content).expect("could not compile UTF8");
    println!("enc: {}",content); 
    Ok(())
}