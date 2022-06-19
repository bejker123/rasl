use std::io::Write;
use std::vec;
use tts::*;
//use tts_rust::{languages::Languages, GTTSClient};

mod default_config;
mod twitch;
use twitch::*;
mod files;
use files::*;
mod cli;
use cli::*;

extern crate chrono;
use chrono::prelude::*;
#[allow(dead_code)]
//That's just to
//safe to delete
fn test_colors() {
    for x in 0..256 {
        println!("\x1b[{0}m test {0}\x1b[0", x);
    }
    loop {}
}

fn get_nr_of_days_in_month(x: i128, year: i128) -> i128 {
    match x {
        2 => {
            //check for leap year
            let mut days = 28_i128;
            if year % 4 == 0 {
                days += 1
            }
            days
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[test]
fn test_nr_of_days_in_a_month() {
    assert_eq!(get_nr_of_days_in_month(1, 2020), 31);
    assert_eq!(get_nr_of_days_in_month(2, 2020), 29);
    assert_eq!(get_nr_of_days_in_month(2, 2021), 28);
    assert_eq!(get_nr_of_days_in_month(3, 2020), 31);
    assert_eq!(get_nr_of_days_in_month(4, 2020), 30);
    assert_eq!(get_nr_of_days_in_month(5, 2020), 31);
    assert_eq!(get_nr_of_days_in_month(6, 2020), 30);
    assert_eq!(get_nr_of_days_in_month(7, 2020), 31);
    assert_eq!(get_nr_of_days_in_month(8, 2020), 31);
    assert_eq!(get_nr_of_days_in_month(9, 2020), 30);
    assert_eq!(get_nr_of_days_in_month(10, 2020), 31);
    assert_eq!(get_nr_of_days_in_month(11, 2020), 30);
    assert_eq!(get_nr_of_days_in_month(12, 2020), 31);
}

#[tokio::test]
async fn test_get_id() {
    let paths = get_file_paths();

    let creds = load_creds(paths.creds_file);
    assert_eq!(get_id("bejker321", creds).await.unwrap(), 401738141u32);
}

#[test]
fn test_fs() {}

#[test]
fn test_tts() {
    let mut tts = Tts::default().unwrap();

    tts.speak("Hello, world.", true).unwrap();
}

//TODO: Add more comments.
//TODO: Refactor and cleanup code.
//TODO: Write more tests.
//TODO: Possibly add GUI? And popup windows notifications?
//TODO: Separate tests to another file if possible.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = get_file_paths();

    let mut update_time = 10;

    let parse_args_ret = parse_args(paths.clone());

    let x = parse_args_ret.0;

    let mut user = parse_args_ret.1;

    if x != -1 {
        update_time = x;
    }

    if user == "" {
        user = "bejker321".to_string();
    }

    // let tts: GTTSClient = GTTSClient {
    //       volume: 1.0,
    //        language: Languages::English, // use the Languages enum
    //   };

    let mut tts = Tts::default()?;

    //  tts.speak("Hello, world.", false)?;

    //test_colors();

    let mut creds = load_creds(paths.clone().creds_file);

    let client_id = creds.client_id.clone();

    let oauth = creds.oauth.clone();

    let mut failed_to_load_creds = false;

    if client_id == "" || oauth == "" {
        failed_to_load_creds = true;
    }

    if failed_to_load_creds {
        print!("\x1b[93mFailed to get creds from creds file, trying to get it from the backup file...\x1b[0m");
        //(client_id, oauth) = load_creds(creds_file + ".bak");

        creds = load_creds(paths.creds_file.clone() + ".bak");

        //let client_id = creds.client_id.clone();

        //let oauth = creds.oauth.clone();
    }
    if oauth == "" || client_id == "" {
        println!("\x1b[93mFailed\x1b[0m");
        if client_id == "" {
            println!("\x1b[93mWarning: client-id is not set, this will likely couse an error later on!\x1b[0m");
        }
        if oauth == "" {
            println!("\x1b[93mWarning: oauth is not set, this will likely couse an error later on!\x1b[0m");
        }
    } else if failed_to_load_creds {
        println!("Success\nTry using the -r-c argument.");
    }

    let id = match get_id(user.as_str(), creds.clone()).await {
        Ok(id) => id,
        _ => 0,
    };

    if id == 0 {
        println!("Failed to get id, check your connection or if the credentials are set up right(use -c/--credentials argument)");
        std::process::exit(0);
    }

    #[allow(unused_assignments)]
    let mut follows: (Vec<String>, Vec<String>) = (vec![], vec![]);
    let mut streams: Vec<Stream> = vec![];

    let mut streams_old = streams;

    #[allow(unused_assignments)]
    let mut ids: Vec<String> = vec![];
    let mut names: Vec<String> = vec![];

    let mut names_old = names;

    let mut first_iter = true;

    loop {
        //clear command line
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let fav_users = get_fav_users(paths.clone());

        follows = get_follows(id.to_string(), &client_id, &oauth).await;

        let get_follows_out = follows.clone();

        ids = get_follows_out.0;
        names = get_follows_out.1;

        streams = get_live_streams(ids, &client_id, &oauth).await;

        if streams == Vec::new() {
            println!("get_live_streams() got an invalid responce, retrying...");
            continue;
        }

        let mut to_print = String::new();

        to_print += &format!("\x1b[94m{} \x1b[0m(\x1b[96m{}\x1b[0m)\n", user, id);

        if names != names_old && !first_iter {
            for name in names.clone() {
                if !names_old.contains(&name) {
                    to_print += &(String::from("\x1b[95mnew follow ") + &name + "\n");
                    tts.speak(String::from("new follow ") + &name.replace("_", " "), false)
                        .unwrap();
                }
            }
            for name_old in names_old.clone() {
                if !names.contains(&name_old) {
                    to_print += &(String::from("\x1b[95mnew unfollow ") + &name_old + "\n");
                    tts.speak(
                        String::from("new unfollow ") + &name_old.replace("_", " "),
                        false,
                    )
                    .unwrap();
                }
            }
        }

        if streams != streams_old && !first_iter {
            for stream in &streams {
                if !streams_old.contains(&stream) {
                    to_print +=
                        &(String::from("\x1b[95mnew stream ") + &stream.user_login + "\x1b[0m\n");
                    tts.speak(
                        String::from("new stream ") + &stream.user_login.replace("_", " "),
                        false,
                    )
                    .unwrap();
                }
            }
            for stream_old in &streams_old {
                if !streams.contains(&stream_old) {
                    to_print += &(String::from("\x1b[95mstream end ")
                        + &stream_old.user_login
                        + "\x1b[0m\n");
                    tts.speak(
                        String::from("stream end ") + &stream_old.user_login.replace("_", " "),
                        false,
                    )
                    .unwrap();
                }
            }
        }

        for stream in &streams {
            //if set to true: checking if you are watching all of currentli live followed streamer, othewrise only live fav users
            let check_all = false;

            let mut watching: bool = false;
            let mut prompt = "";
            let if_watching = "\x1b[91mwatching: ";
            if check_all {
                watching = get_viewers(stream.user_login.clone()).await.contains(&user);

                if watching {
                    prompt = "\x1b[91mwatching: ";
                }
            }

            let x = match fav_users.contains(&stream.user_login.clone()) {
                true => {
                    if !check_all {
                        watching = get_viewers(stream.user_login.clone()).await.contains(&user);
                        prompt = if_watching;
                    }
                    if !watching {
                        tts.speak(
                            String::from("fav user ")
                                + &stream.user_login.replace("_", " ")
                                + " is live",
                            false,
                        )
                        .unwrap();
                        "\x1b[91m"
                    } else {
                        prompt
                    }
                }
                _ => prompt,
            };

            let time = stream.started_at.replace("T", " ");
            let time = time.replace("Z", "");
            let time = time.split(" ").collect::<Vec<&str>>();

            let str_vec_to_i128_vec = |x: &str, split_patern: &str| {
                x.split(split_patern)
                    .map(|o| o.parse::<i128>().unwrap())
                    .collect::<Vec<i128>>()
            };

            let utc = Utc::now().to_string();
            let utc = utc.split(".");
            let utc = utc.collect::<Vec<&str>>()[0]
                .split(" ")
                .collect::<Vec<&str>>();

            let bigger_time_now = str_vec_to_i128_vec(utc[0], "-");

            let lesser_time_now = str_vec_to_i128_vec(utc[1], ":");

            let bigger_time = str_vec_to_i128_vec(time[0], "-");

            let lesser_time = str_vec_to_i128_vec(time[1], ":");

            let mut output_time: Vec<i128> = Vec::new();

            for i in 0..bigger_time.len() {
                let tmp = bigger_time_now[i] - bigger_time[i];
                output_time.push(tmp);
            }

            for i in 0..lesser_time.len() {
                let tmp = lesser_time_now[i] - lesser_time[i];
                output_time.push(tmp);
            }

            //format: years [0], months [1], days [2]; hours [3], minutes [4], seconds [5]

            //handle underflow for:
            //seconds
            if output_time[5] < 0 {
                output_time[5] += 60;
                output_time[4] -= 1;
            }
            //minutes
            if output_time[4] < 0 {
                output_time[4] += 60;
                output_time[3] -= 1;
            }
            //hours
            if output_time[3] < 0 {
                output_time[3] += 24;
                output_time[2] -= 1;
            }
            //days
            if output_time[2] < 0 {
                //get the number of days in each month
                output_time[2] += get_nr_of_days_in_month(bigger_time_now[1], bigger_time_now[0]);
                output_time[1] -= 1;
            }
            //months
            if output_time[1] < 0 {
                output_time[1] += 12;
                output_time[0] -= 1;
            }

            let mut o_time = String::new();

            if output_time[0] != 0 {
                //tts.speak(format!("{} has been live for {} years",stream.user_login,output_time[0]),false).unwrap();
                o_time += &(output_time[0].to_string() + " years");
            }
            if output_time[1] != 0 {
                //tts.speak(format!("{} has been live for {} months",stream.user_login,output_time[1]),false).unwrap();
                o_time += &(output_time[1].to_string() + " months");
            }
            if output_time[2] != 0 {
                //tts.speak(format!("{} has been live for {} days",stream.user_login,output_time[2]),false).unwrap();
                o_time += &(output_time[2].to_string() + " days");
            }

            for i in 3..output_time.len() {
                let mut s = output_time[i].to_string();
                if s.len() == 1 {
                    let x = format!("0{}", s);
                    s = x;
                }
                o_time += &s;
                if i != output_time.len() - 1 {
                    o_time += ":";
                }
            }

            to_print += &format!(
                "\x1b[93m{}\x1b[0m {}{} \x1b[92m{} \x1b[96m\"{}\"\x1b[0m \x1b[92m{}\x1b[0m\n",
                stream.viewer_count, x, stream.user_name, stream.game_name, stream.title, o_time
            );
        }

        names_old = names;
        streams_old = streams;

        print!("{}", to_print);
        let mut frame = 0;
        let chars = vec!['|', '/', '-', '\\'];
        for i in 0..update_time + 1 {
            let char = chars[frame];
            frame += 1;
            if frame > chars.len() - 1 {
                frame = 0;
            }
            if i == 1 {
                print!("\r{}", " ".repeat(15));
            }
            print!("\r{} {}/{}", char, i, update_time);
            if i == 0 {
                print!(" updated!");
            }
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        first_iter = false;
    }
    #[allow(unreachable_code)]
    Ok(())
}
