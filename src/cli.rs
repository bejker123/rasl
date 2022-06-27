use std::fs;

use crate::files::Paths;

//this function displays help and quits
pub fn display_help() {
    let first_line = format!("Usage: {}", std::env::args().collect::<Vec<String>>()[0]);
    //first line lenght up to bracket
    let fllutb = first_line.len();
    let padding = " ".repeat(fllutb);
    println!("{} [options]", first_line);
    println!(
        "{} -c/--credentials <client-id> <oauth> save credentials.",
        padding
    );
    println!(
        "{} -r-c/--restore-credentials try to restore credentials from backup file.",
        padding
    );
    println!(
        "{} -t/--time <secs> set update time.",
        padding
    );
    println!(
        "{} -u/--user <secs> set user.",
        padding
    );
    println!("{} -h/--help display help.", padding);
    std::process::exit(1);
}

fn invalid_argument_usage(arg : String){
    println!("Invalid argument usage: {}", arg);
    display_help();
}

pub fn parse_args(paths : Paths) ->(i32,String){
    let mut skip_next = 0 as usize;
    let args = std::env::args().collect::<Vec<String>>();
    let sliced_args = &args[1..args.len()];
    let mut update_time = -1;
    let mut user = String::new();
    for i in 0..sliced_args.len() {
        let arg = &sliced_args[i];
        if skip_next > 0 {
            skip_next -= 1;
            continue;
        }
        let args_len = sliced_args.len()-1;//adding the +1 for the checks
        match arg.to_lowercase().as_str() {
            "-u"|"--user"=>{
                skip_next = 1;
                if args_len - i < 1 {//here
                    invalid_argument_usage(arg.to_string());
                }
                user = args[i+2].clone();
               // println!("{}",user);
            },
            "-t"|"--time"=>{
                skip_next = 1;
                if args_len - i < 1 {
                    invalid_argument_usage(arg.to_string());
                }
                match args[i+2].parse::<i32>(){
                    Ok(o)=>{
                        println!("Set update time to {} seconds",o);
                        update_time = o},
                    _=>{invalid_argument_usage(arg.to_string());}
                };
            },
            "-r-c" | "--restore-credentials" => {
                if fs::metadata(paths.creds_file.clone() + ".bak").is_ok() {
                    //check if creds backup file exists
                    fs::rename(paths.creds_file.clone(), paths.creds_file.clone() + ".bak1").unwrap();
                    fs::rename(paths.creds_file.clone() + ".bak", paths.creds_file.clone()).unwrap();
                    fs::rename(paths.creds_file.clone() + ".bak1", paths.creds_file.clone() + ".bak").unwrap();
                } else {
                    fs::copy(paths.creds_file.clone(), paths.creds_file.clone() + ".bak").unwrap();
                    println!("\x1b[93mBackup credentials file doesn't exist, creating one!\x1b[0m")
                }
            }
            "-c" | "--credentials" => {
                skip_next = 2;
                if args_len - i < 2 {
                    invalid_argument_usage(arg.to_string());
                }
                let client_id = &sliced_args[i + 1];
                let oauth = &sliced_args[i + 2];
                if !fs::metadata(paths.creds_file.clone()).is_ok() {
                    //check if creds file exists
                    fs::rename(paths.creds_file.clone(), paths.creds_file.clone() + ".bak").unwrap();
                }
                match fs::write(paths.creds_file.clone(),format!("{}\r\n{}",client_id,oauth)){
                    Ok(_)=>{},
                    Err(e)=>println!("\x1b[93mFailed to write to credentials file({})\x1b[0m\n^^^Acctual error:{}",paths.creds_file,e)
                }
            }
            "-h" | "--help" => display_help(),
            _ => {
                println!("Invalid argument: {}", arg);
                display_help()
            } //TODO: add "here ^"
        }
    }
    (update_time,user)
}
