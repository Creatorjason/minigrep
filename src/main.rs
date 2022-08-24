#![allow(unused)]
// get user input from terminal
use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::Config;

// struct Config{
    // query : String,
    // file_path : String
// }
// impl Config{
    // fn build(args: &Vec<String>) -> Result<Config, String> {
        // if args.len() < 3{
            // return Err("Not enough arguments".to_string())
        // }
        // Ok(
        // Config{
            // query : args[0].clone(),
            // file_path : args[2].clone(),
        // })
    // }
// }
fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
});
    
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1)
    };


}
// fn parse_config(config: &Vec<String>) -> Config{
    // let query = config[0].clone();
    // let file_path = config[2].clone();
    // Config { query: (query), file_path: (file_path) }
// }
// fn run(config: Config) -> Result<(), Box<dyn Error>>{
//     let file: String = fs::read_to_string(config.file_path)?;
//     println!("{}",file);
//     Ok(())
// }