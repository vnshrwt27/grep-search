use std::{env, io::{BufRead, Write, stdin, stdout}};
fn main() {
    let stdin = stdin();
    let pattern = env::args() 
        .nth(1).unwrap();
    let mut stdout = stdout() ;
    for line in stdin.lock().lines(){
        let line = line.expect("Error reading line") ;

        if line.contains(&pattern){
            write!(stdout,"{}\n",line).unwrap();
            stdout.flush().unwrap();

        }
    }
         

}
