extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

#[path = "default_config.rs"]
mod default_config;

//returns true if created, false if already existed
pub fn ensure_dir_exists(dir: String) -> bool {
    if !fs::metadata(dir.clone()).is_ok() {
        fs::create_dir(dir).unwrap();
        return true;
    }
    false
}

//returns true if created, false if already existed
pub fn ensure_file_exists(path: String) -> bool {
    if !fs::metadata(path.clone()).is_ok() {
        fs::File::create(path).unwrap();
        return true;
    }
    false
}


#[allow(dead_code)]
//old function safe to delete
pub fn init_credentials() -> (String, String) {
    dotenv().ok();

    let client_id = match env::var("client_id") {
        Ok(key) => key,
        Err(e) => panic!("Initing credentials failed: {}", e),
    };

    let oauth = match env::var("oauth") {
        Ok(key) => key,
        Err(e) => panic!("Initing credentials failed: {}", e),
    };

    (client_id, oauth)
}

//sets up the user svae directory
pub fn setup_save_dir(save_path: String, fav_users_file: String, creds_file: String) {
    ensure_dir_exists(save_path.clone());
    ensure_file_exists(creds_file.clone());
    if ensure_file_exists(fav_users_file.clone()) {
        let mut file = fs::File::create(fav_users_file.clone()).unwrap();
        let mut content = String::new();
        let default_fav_users = default_config::default_fav_users();
        for i in 0..default_fav_users.len() {
            content += &default_fav_users[i].clone();
            if i != default_fav_users.len() - 1 {
                content += "\r\n";
            }
        }
        file.write_all(content.as_bytes()).unwrap();
    }
}

//Reads fav_users_file into a vector of Strings
pub fn get_fav_users(fav_users_file: String) -> Vec<String> {
    let mut file = fs::File::open(fav_users_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    //this is really ugly but works...
    content
        .split("\r\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.into())
        .collect()
}

//used at startup to load credentials from the save directory
pub fn get_creds(creds_file: String) -> (String, String) {
    let mut file = fs::File::open(creds_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    //this is really ugly but works...
    let mut creds = content
        .split("\r\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.into())
        .collect::<Vec<String>>();
    if creds.len() < 2 {
        if creds.len() > 1 {
            creds.push("".to_string());
        }
        creds.push("".to_string());
    } else if creds.len() > 2 {
        println!(
            "\x1b[93mWarning: creds file contains more data than needed({}>2 lines).\x1b[0m",
            creds.len()
        );
        println!("^^^^ Try using --credentials argument to fix the problem above.");
    }
    (creds[0].clone(), creds[1].clone())
}
