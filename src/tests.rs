use tts::*;

use crate::{get_days_in_month, files::*, twitch::*,cli::*};

#[test]
fn test_nr_of_days_in_a_month() {
    assert_eq!(get_days_in_month(1, 2020), 31);
    assert_eq!(get_days_in_month(2, 2020), 29);
    assert_eq!(get_days_in_month(2, 2021), 28);
    assert_eq!(get_days_in_month(3, 2020), 31);
    assert_eq!(get_days_in_month(4, 2020), 30);
    assert_eq!(get_days_in_month(5, 2020), 31);
    assert_eq!(get_days_in_month(6, 2020), 30);
    assert_eq!(get_days_in_month(7, 2020), 31);
    assert_eq!(get_days_in_month(8, 2020), 31);
    assert_eq!(get_days_in_month(9, 2020), 30);
    assert_eq!(get_days_in_month(10, 2020), 31);
    assert_eq!(get_days_in_month(11, 2020), 30);
    assert_eq!(get_days_in_month(12, 2020), 31);
}

#[tokio::test]
async fn test_get_id() {
    let paths = get_file_paths();

    let ns = String::new();

    //testing fs at the same time
    assert_ne!(paths.creds_file, ns);
    assert_ne!(paths.fav_users_file, ns);
    assert_ne!(paths.save_path, ns);

    let creds = load_creds(paths.creds_file);
    assert_eq!(get_id("bejker321", creds).await.unwrap(), 401738141u32);
}

//#[test]
#[tokio::test]
async fn test_tts() {
    let mut tts = Tts::default().unwrap();

    tts.speak("Hello, world.", true).unwrap();
}

#[test]
fn test_ensure_exists(){
    assert_eq!(does_exist(String::from("")),false);
    assert_eq!(does_exist(String::from("Cargo.toml")),true);

    assert_eq!(ensure_dir_exists(String::from("..")),false);
    assert_eq!(ensure_file_exists(String::from("Cargo.toml")),false);

    let test_dir = "test_dir";
    let test_file = "test_file";

    std::fs::remove_dir(test_dir);
    assert_eq!(does_exist(String::from(test_dir)),false);
    assert_eq!(ensure_dir_exists(String::from(test_dir)),true);
    
    std::fs::remove_file(test_file);
    assert_eq!(does_exist(String::from(test_file)),false);
    assert_eq!(ensure_file_exists(String::from(test_file)),true);

    std::fs::remove_dir(test_dir).unwrap();
    std::fs::remove_file(test_file).unwrap();
}