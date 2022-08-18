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

fn readfile(file:&String) -> Result<String,Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)

}

//TODO refactor to accept longer keys maybe break into multiple functions
pub fn scramble(config: &Config) -> Result<String,&'static str> {
    let content = readfile(&config.file).unwrap();
    if config.argument == "F" {
        return Err("TRIGGERED BY TYPE F")
    }
    println!("not enc: {}",content);
    let b_content =  content.as_bytes();
    let key_vector = config.key.as_bytes();
    let mut nb_content: Vec<u32> = Vec::new();
    let mut i = 0 ;
    let mut k = 0 ;
    while i < b_content.len() {
        if k >= key_vector.len() {
            k = 0;
            nb_content.push(u32::from(b_content[i]) + u32::from(key_vector[k]));
            i+=1;
            k+=1;  
        } else {
            nb_content.push(u32::from(b_content[i]) + u32::from(key_vector[k]));
            i+=1;
            k+=1;  
        }

    }
    let mut content = String::new();
    for item in nb_content {
        
        content.push_str("#");
        content.push_str(&item.to_string());
    }
    Ok(content)
}

fn read_to_array (content: &String) ->Result<Vec<u32>,&'static str> {
    if content == "" {
        return Err("Given an Empty String");
    }
    let mut in_array: Vec<u32> = Vec::new();
    let v: Vec<&str>  = content.split("#").collect();

    for item in v {
        in_array.push(item.parse::<u32>().unwrap());
    }

    Ok(in_array)
}

pub fn un_scramble(config: &Config) -> Result<String,&'static str> {
    let content = readfile(&config.file).unwrap();
    if config.argument == "F" {
        return Err("TRIGGERED BY TYPE F")
    }
    println!("not enc: {}",content);
    let b_content =  read_to_array(&content).unwrap();
    let key_vector = config.key.as_bytes();
    let mut nb_content: Vec<u32> = Vec::new();
    let mut i = 0 ;
    let mut k = 0 ;
    while i < b_content.len() {
        if k >= key_vector.len() {
            k = 0;
            nb_content.push(b_content[i] - u32::from(key_vector[k]));
            i+=1;
            k+=1;  
        } else {
            nb_content.push(b_content[i] - u32::from(key_vector[k]));
            i+=1;
            k+=1;  
        }

    }
    // TODO turn nb_content into u8 and then back into string
    let mut content = String::new();
    for item in nb_content {
        
        content.push_str("#");
        content.push_str(&item.to_string());
    }
    Ok(content)
}