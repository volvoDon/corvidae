use std::error::Error;
use std::fs; 

pub struct Config <'a> {
    pub key: &'a String,
    pub file: &'a String,
    pub argument: &'a String,
}

impl Config <'_> {
    pub fn new (args : &Vec<String>) -> Result<Config,&'static str> {
        if args.len() < 4 {
            return Err("not enough arguments")
        }
        
        let key = &args[1];
        let file = &args[2];
        let argument = &args[3];
        Ok(Config{key,file,argument})
    }    
}

pub fn readfile(file:&String) -> Result<String,Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)

}

pub fn read_to_array (content: &String) ->Result<Vec<u32>,&'static str> {
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
pub fn write_to_file (content: &String,config: &Config){
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
        let png_info = steganometry::read_png(config);
        println!("png_info: {:?}",png_info.info);
        println!("bit_length: {:?}",png_info.bytes.len());
        steganometry::write_png(config, &png_info)
    } else {
        println!("incorrect Argument (-e for encrypt, -d for decrypt, -p for stegometric png manipulation)")
    }
}

mod steganometry {
    use png;
    use std::fs;
    use std::path::Path;
    use std::io::{BufWriter};
    use crate::Config;
    //only public for now to print
    pub struct PngInfo {
        pub info:  png::OutputInfo,
        pub bytes: Vec<u8>
    }
    pub fn read_png (config: &Config) -> PngInfo {
        let decoder = png::Decoder::new(fs::File::open(config.file.clone()).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        
        PngInfo {info: info, bytes: bytes.to_vec()}
    }
   
    fn new_datatable (pnginfo: &PngInfo, config: &Config) -> Vec<u8> {
        let message = crate::readfile(config.key).expect("could not open file");
        let data = crate::read_to_array(&message).expect("Failed to push string to array, Likely you did not encrypt");
        println!("len: {}",data.len());
        let mut table : Vec<u8> = Vec::new();
        if pnginfo.info.color_type == png::ColorType::Rgb {
            let mut cnt = 0;
            for (i, el) in pnginfo.bytes.iter().enumerate() {
                if (i+1)%3 == 0 && i != 0 {
                    table.push(*el as u8);
                    if cnt < data.len() {
                    table.push(data[cnt] as u8);
                    cnt += 1} else {table.push(0)}    
                } else {
                    table.push(*el);
                }

            }
        
            return table   
        } else if pnginfo.info.color_type == png::ColorType::Rgba {
            for (i, el) in pnginfo.bytes.iter().enumerate() {
                println!("The current element is {}", el);
                println!("The current index is {}", i);     
                
            }
            return table    
        } else {
        println!("png is not rgb or rgba");
        panic!()
        }
    }
    pub fn write_png (config: &Config, pnginfo: &PngInfo) {

        let file = fs::File::open(config.file.clone()).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w,pnginfo.info.width, pnginfo.info.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();
        let data_table = new_datatable(pnginfo,config);
        for i in 1500..2000 {
            println!("data {} , {} ",data_table[i],i);   
        }
        writer.write_image_data(&data_table).unwrap();

    }
}
//TODO use this info to write a png with hidden data

    
