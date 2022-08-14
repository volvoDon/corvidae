use std::env;
use corvidae::Config;


fn main() {
    let config : Vec<String> = env::args().collect();
    let config = Config::new(&config).unwrap();
    println!("key: {}",config.key);
    println!("file: {}",config.file);
    println!("argument: {}",config.argument);

    let mut content = corvidae::readfile(&config.file).unwrap();
    corvidae::scramble(&mut content, &config).unwrap();
}


