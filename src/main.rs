use std::{env, fs, io::{BufRead, Write, stdin, stdout}};
fn main() {

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args) ;
    println!("{:?}",&config);
    run(&config);
}
         

#[derive(Debug)]
struct Config{
    query : String ,
    file_path : String,
}
fn parse_config(args : &[String]) -> Config {
    let query = args[1].clone() ;
    let file_path = args[2].clone() ;

    Config { query, file_path }
    
}
fn run(config : &Config) {
    let content = fs::read_to_string(&config.file_path)
        .expect("Unable to read file");
    for line in content.lines(){
        if line.contains(&config.query){
            writeln!(stdout(),"{}",line).unwrap();
            stdout().flush().unwrap();
        }
    }
}
