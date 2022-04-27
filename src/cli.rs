use std::fs;

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
        "{} --r-c/restore-credentials try to restore credentials from backup file.",
        padding
    );
    println!("{} -h/--help display help.", padding);
    std::process::exit(1);
}

pub fn parse_args(creds_file: String) {
    let mut skip_next = 0 as usize;
    let args = std::env::args().collect::<Vec<String>>();
    let sliced_args = &args[1..args.len()];
    for i in 0..sliced_args.len() {
        let arg = &sliced_args[i];
        if skip_next > 0 {
            skip_next -= 1;
            continue;
        }
        match arg.to_lowercase().as_str() {
            "-r-c" | "restore-credentials" => {
                if fs::metadata(creds_file.clone() + ".bak").is_ok() {
                    //check if creds backup file exists
                    fs::rename(creds_file.clone(), creds_file.clone() + ".bak1").unwrap();
                    fs::rename(creds_file.clone() + ".bak", creds_file.clone()).unwrap();
                    fs::rename(creds_file.clone() + ".bak1", creds_file.clone() + ".bak").unwrap();
                } else {
                    fs::copy(creds_file.clone(), creds_file.clone() + ".bak").unwrap();
                    println!("\x1b[93mBackup credentials file doesn't exist, creating one!\x1b[0m")
                }
            }
            "-c" | "--credentials" => {
                skip_next = 2;
                if sliced_args.len() - i < i + 2 {
                    println!("Invalid argument usage: {}", arg);
                    display_help();
                }
                let client_id = &sliced_args[i + 1];
                let oauth = &sliced_args[i + 2];
                if !fs::metadata(creds_file.clone()).is_ok() {
                    //check if creds file exists
                    fs::rename(creds_file.clone(), creds_file.clone() + ".bak").unwrap();
                }
                match fs::write(creds_file.clone(),format!("{}\r\n{}",client_id,oauth)){
                    Ok(_)=>{},
                    Err(e)=>println!("\x1b[93mFailed to write to credentials file({})\x1b[0m\n^^^Acctual error:{}",creds_file,e)
                }
            }
            "-h" | "--help" => display_help(),
            _ => {
                println!("Invalid argument: {}", arg);
                display_help()
            } //TODO: add "here ^"
        }
    }
}
