use std::env;
use corvidae::Config;


fn main() {
    let config : Vec<String> = env::args().collect();
    let config = Config::new(&config).unwrap();
    println!("key: {}",config.key);

    corvidae::run(&config); 

}


