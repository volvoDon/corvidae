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
fn write_to_file (content: &String,config: &Config){
    let path = config.file.clone();
    fs::write(path, content).expect("could not write file");
} 

//TODO refactor to accept longer keys maybe break into multiple functions
pub fn scramble(config: &Config) -> Result<(),&'static str> {
    let content = readfile(&config.file).unwrap();
    if config.argument == "F" {
        return Err("TRIGGERED BY TYPE F")
    }
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
    content.remove(0);
    write_to_file(&content, config);
    Ok(())
}

pub fn un_scramble(config: &Config) -> Result<(),&'static str> {
    let content = readfile(&config.file).expect("file could not be read");
    if config.argument == "F" {
        return Err("TRIGGERED BY TYPE F")
    }
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
    
    let mut nb_content8 = Vec::new();
    for item in nb_content {
        nb_content8.push(item as u8);
    }
    let content = String::from_utf8(nb_content8).expect("string from utf8 failed");
    write_to_file(&content, config);
    Ok(())
        
}

pub fn run (config: &Config) {
    if config.argument == "-e" {
        scramble(config).unwrap();
    } else if config.argument == "-d" {
        un_scramble(config).unwrap();         
    } else if config.argument == "-p" {
        steganometry::read_png(config);
    } else {
        println!("incorrect Argument (-e for encrypt, -d for decrypt)")
    }
}

mod steganometry {
    use png;
    use std::fs;
    use crate::Config;
    pub fn read_png (config: &Config) {
        let decoder = png::Decoder::new(fs::File::open(config.file.clone()).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        println!("png_info: {:?}",info);
        println!("bit_length: {:?}",bytes.len());
    }
}
//TODO use this info to write a png with hidden data

    
