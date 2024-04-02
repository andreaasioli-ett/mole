use std::env;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("too many params");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_prompt(){

    loop {

        print!(":> ");
        io::stdout().flush().unwrap();
    
        let mut line = String::new();
        
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
    
        if  line.to_lowercase() == "exit\n"{
            break;
        } else if line != ""{
            scan(line)
        } else {
            continue;
        }
        
    }


}

fn run_file(path: &String){
    println!("file path {}", path);
}

fn scan(script: String){
    println!("eccolo: {}", script)
}