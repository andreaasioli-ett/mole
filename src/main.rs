use std::env;
use std::io;
use std::io::Write;
use std::fs;

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
            .expect("failed to read line");

        line.truncate(line.len() - 1);
    
        if  line.to_lowercase() == "exit"{
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

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

}

fn scan(script: String){
    println!("eccolo: {}", script)
}