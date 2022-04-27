use std::io::Write;
use std::vec;
use tts_rust::{languages::Languages, GTTSClient};

mod default_config;
mod twitch;
use twitch::*;
mod files;
use files::*;
mod cli;
use cli::*;

#[allow(dead_code)]
//That's just to
//safe to delete
fn test_colors() {
    for x in 0..256 {
        println!("\x1b[{0}m essa {0}\x1b[0", x);
    }
    loop {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let save_path = String::from(env!("LOCALAPPDATA")) + "/" + env!("CARGO_PKG_NAME");
    let fav_users_file = save_path.clone() + "/fav_users.txt";
    //let _follows_file = save_path.clone() + "/follows.txt";
    let _events_file = save_path.clone() + "/events.txt";
    let creds_file = save_path.clone() + "/creds";
    //let _config_file = save_path.clone() + "/config.cfg";

    parse_args(creds_file.clone());

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
    };

    //test_colors();
    setup_save_dir(
        save_path.clone(),
        fav_users_file.clone(),
        creds_file.clone(),
    );

    let (mut client_id, mut oauth) = get_creds(creds_file.clone());
    let mut failed_to_get_creds = false;

    if client_id == "" || oauth == "" {
        failed_to_get_creds = true;
    }

    if failed_to_get_creds {
        print!("\x1b[93mFailed to get creds from creds file, trying to get it from the backup file...\x1b[0m");
        (client_id, oauth) = get_creds(creds_file + ".bak");
    }
    if oauth == "" || client_id == "" {
        println!("\x1b[93mFailed\x1b[0m");
        if client_id == "" {
            println!("\x1b[93mWarning: client-id is not set, this will likely couse an error later on!\x1b[0m");
        }
        if oauth == "" {
            println!("\x1b[93mWarning: oauth is not set, this will likely couse an error later on!\x1b[0m");
        }
    } else if failed_to_get_creds {
        println!("Success\nTry using the -r-c argument.");
    }

    let user = "<your username here>";

    let id = match get_id(user, &client_id, &oauth).await {
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
        let fav_users = get_fav_users(fav_users_file.clone());

        follows = get_follows(id.to_string(), &client_id, &oauth).await;

        (ids, names) = follows.clone();

        streams = get_live_streams(ids, &client_id, &oauth).await;

        let mut to_print = String::new();

        to_print += &format!("\x1b[94m{} \x1b[0m(\x1b[96m{}\x1b[0m)\n", user, id);

        if names != names_old && !first_iter {
            for name in names.clone() {
                if !names_old.contains(&name) {
                    to_print += &(String::from("\x1b[95mnew follow ") + &name + "\n");
                    narrator.speak(&(String::from("new follow ") + &name));
                }
            }
            for name_old in names_old.clone() {
                if !names.contains(&name_old) {
                    to_print += &(String::from("\x1b[95mnew unfollow ") + &name_old + "\n");
                    narrator.speak(&(String::from("new unfollow ") + &name_old));
                }
            }
        }

        if streams != streams_old && !first_iter {
            for stream in &streams {
                if !streams_old.contains(&stream) {
                    to_print += &(String::from("\x1b[95mnew stream ") + &stream.user_login + "\n");
                    narrator.speak(&(String::from("new stream ") + &stream.user_login));
                }
            }
            for stream_old in &streams_old {
                if !streams.contains(&stream_old) {
                    to_print += &(String::from("\x1b[stream end ") + &stream_old.user_login + "\n");
                    narrator.speak(&(String::from("stream end ") + &stream_old.user_login));
                }
            }
        }

        for stream in &streams {
            let x = match fav_users.contains(&stream.user_login) {
                true => {
                    narrator.speak(&(String::from("fav user ") + &stream.user_login + " is live"));
                    "\x1b[91m"
                }
                _ => "",
            };

            to_print += &format!(
                "\x1b[93m{}\x1b[0m {}{} \x1b[92m{} \x1b[96m\"{}\"\x1b[0m\n",
                stream.viewer_count, x, stream.user_name, stream.game_name, stream.title
            );
        }

        names_old = names;
        streams_old = streams;

        print!("{}", to_print);
        let mut frame = 0;
        let chars = vec!['|', '/', '-', '\\'];
        let update_time = 10;
        for i in 0..update_time + 1 {
            let char = chars[frame];
            frame += 1;
            if frame > chars.len() - 1 {
                frame = 0;
            }
            print!("\r{} {}/{}", char, i, update_time);
            if i == 0 {
                print!(" updated!");
            } else if i == 1 {
                print!("{}", " ".repeat(9));
            }
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        first_iter = false;
    }
    #[allow(unreachable_code)]
    Ok(())
}
