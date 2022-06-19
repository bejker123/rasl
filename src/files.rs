extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

#[path = "default_config.rs"]
mod default_config;

#[path = "structs.rs"]
mod structs;

pub use structs::*;

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

pub fn get_file_paths() -> Paths {
    let mut save_path = String::new();

    if std::env::consts::OS == "windows" {
        save_path =
            String::from(std::env::var("LOCALAPPDATA").unwrap() + "/" + env!("CARGO_PKG_NAME"));
    } else {
        save_path = String::from(
            std::env::var("HOME").unwrap() + "/." + &std::env::var("CARGO_PKG_NAME").unwrap(),
        );
    }
    // println!("save path: {}",save_path);
    let fav_users_file = save_path.clone() + "/fav_users.txt";
    //let _follows_file = save_path.clone() + "/follows.txt";
    let _events_file = save_path.clone() + "/events.txt";
    let creds_file = save_path.clone() + "/creds";
    //let _config_file = save_path.clone() + "/config.cfg";

    let paths = Paths {
        save_path,
        fav_users_file,
        creds_file,
    };

    setup_save_dir(paths.clone());
    paths
}

#[allow(dead_code)]
//old function safe to delete
pub fn init_credentials() -> Creds {
    dotenv().ok();

    let client_id = match env::var("client_id") {
        Ok(key) => key,
        Err(e) => panic!("Initing credentials failed: {}", e),
    };

    let oauth = match env::var("oauth") {
        Ok(key) => key,
        Err(e) => panic!("Initing credentials failed: {}", e),
    };

    Creds { client_id, oauth }
}

//sets up the user svae directory
pub fn setup_save_dir(paths: Paths) {
    ensure_dir_exists(paths.save_path.clone());
    ensure_file_exists(paths.creds_file.clone());
    // ensure_file_exists(creds_file.clone()+".bak");
    if ensure_file_exists(paths.fav_users_file.clone()) {
        let mut file = fs::File::create(paths.fav_users_file.clone()).unwrap();
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
pub fn get_fav_users(paths: Paths) -> Vec<String> {
    let mut file = fs::File::open(paths.fav_users_file).unwrap();
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
pub fn load_creds(creds_file: String) -> Creds {
    //println!("{}",creds_file);
    ensure_file_exists(creds_file.clone());
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
    Creds {
        client_id: creds[0].clone(),
        oauth: creds[1].clone(),
    }
}
