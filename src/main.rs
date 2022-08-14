use std::env;
use corvidae::Config;


fn main() {
    let config : Vec<String> = env::args().collect();
    let config = Config::new(&config).unwrap();
    println!("key: {}",config.key);
    println!("file: {}",config.file);
    println!("argument: {}",config.argument);

    let content = corvidae::readfile(&config.file).unwrap();
    corvidae::scramble(&content, &config).unwrap();
}


