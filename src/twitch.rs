use json::JsonValue;

use crate::files::Creds;

extern crate json;
extern crate reqwest;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Stream {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    pub type_: String,
    pub title: String,
    pub viewer_count: u128,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub tag_ids: Vec<String>,
    pub is_mature: bool,
}
impl PartialEq for Stream {
    fn eq(&self, other: &Stream) -> bool {
        //self.memory.eq(other)
        self.id == other.id
            && self.user_login == other.user_login
            && self.user_name == other.user_name
    }
}

//Function to get id of a twitch user.
pub async fn get_id(user: &str, creds: Creds) -> Result<u32, u32> {
    let client = reqwest::Client::new();
    let mut _is_allive = true;
    // let mut retries: u32 = 0;
    let mut text = String::from("get_id() failed to connect");
    let url = String::from("https://api.twitch.tv/helix/users?login=") + user;
    while _is_allive {
        // if retries >= 5 {
        //       break;
        //  }
        let res = match client
            .get(url.clone())
            .header(
                reqwest::header::AUTHORIZATION,
                String::from("Bearer ") + &creds.oauth,
            )
            .header("Client-ID", creds.client_id.clone())
            .send()
            .await
        {
            Ok(a) => {
                _is_allive = false;
                a
            }
            Err(_e) => {
                //retries += 1;
                continue;
            }
        };
        text = res.text().await.unwrap();
        //  println!("{}",text);
        let j = json::parse(&text).unwrap();
        // println!("{}",res.text().await.unwrap());
        return match j["data"][0]["id"].to_string().parse::<u32>() {
            Ok(o) => Ok(o),
            _ => {
                println!("get_id(url={}): Error: {}", url, text);
                Err(0)
            }
        };
    }
    println!("get_id(url={}): Error: {}", url, text);
    Err(0)
}

//Requests follows from the twitch api
pub async fn get_follows(id: String, client_id: &str, oauth: &str) -> (Vec<String>, Vec<String>) {
    let mut ids: Vec<String> = vec![];
    let mut names: Vec<String> = vec![];
    let client = reqwest::Client::new();
    let mut cursor = String::new();
    loop {
        let res = match client
            .get(format!(
                "https://api.twitch.tv/helix/users/follows?from_id={}&first={}&after={}",
                id, 100, cursor
            ))
            .header(
                reqwest::header::AUTHORIZATION,
                String::from("Bearer ") + oauth,
            )
            .header("Client-ID", client_id)
            .send()
            .await
        {
            Ok(a) => a,
            Err(_e) => continue,
        };
        let text = res.text().await.unwrap();
        let j = json::parse(&text).unwrap();
        let data = j["data"].to_owned();
        for i in 0..data.len() {
            let follow = &data[i];
            ids.push(follow["to_id"].to_string());
            names.push(follow["to_login"].to_string());
        }
        let cursor_ = &j["pagination"]["cursor"];
        cursor = cursor_.dump().replace('"', "");
        if cursor == "null" {
            break;
        }
    }
    (ids, names)
}

async fn get_lives_from_ids(
    ids: Vec<String>,
    client_id: &str,
    oauth: &str,
) -> Result<Vec<Stream>, String> {
    let mut out_streams: Vec<Stream> = vec![];
    let mut ids_string = String::new();
    //format
    for id in ids {
        ids_string += &(String::from("user_id=") + &id + "\u{0026}");
    }
    ids_string = ids_string[0..ids_string.len() - 1].to_string();
    let client = reqwest::Client::new();
    #[allow(unused_assignments)]
    let mut res = String::new();

    match client
        .get(format!(
            "https://api.twitch.tv/helix/streams?first=100&{}",
            ids_string
        ))
        .header(
            reqwest::header::AUTHORIZATION,
            String::from("Bearer ") + oauth,
        )
        .header("Client-ID", client_id)
        .send()
        .await
    {
        Ok(r) => res = r.text().await.unwrap(),
        Err(e) => {
            return Err(String::from(
                "get_lives_from_ids() got an invalid resonce!\nAcctual error: ",
            ) + &e.to_string());
        }
    };

    let j = json::parse(&res).unwrap();
    // println!("{:?}",j.dump());
    let streams = &j["data"];
    for i in 0..streams.len() {
        let stream = &streams[i];
        let mut tag_ids = stream["tag_ids"].dump().replace('"', "");
        tag_ids = tag_ids[1..tag_ids.len() - 1].to_string();
        let tag_ids = tag_ids
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|&x| x.into())
            .collect();
        //Yeah, I sure hope there is a better way, but for now this works.
        let stream = Stream {
            id: stream["id"].to_string(),
            user_id: stream["user_id"].to_string(),
            user_login: stream["user_login"].to_string(),
            user_name: stream["user_name"].to_string(),
            game_id: stream["game_id"].to_string(),
            game_name: stream["game_name"].to_string(),
            type_: stream["type"].to_string(),
            title: stream["title"].to_string(),
            viewer_count: stream["viewer_count"].to_string().parse::<u128>().unwrap(),
            started_at: stream["started_at"].to_string(),
            language: stream["language"].to_string(),
            thumbnail_url: stream["thumbnail_url"].to_string(),
            tag_ids,
            is_mature: stream["is_mature"].as_bool().unwrap(),
        };
        out_streams.push(stream);
    }

    Ok(out_streams)
}

fn get_viewer_cathegory(cathegory: &JsonValue) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in 0..cathegory.len() {
        out.push(cathegory[i].to_string().to_owned());
    }
    out
}

pub async fn get_viewers(user: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let client = reqwest::Client::new();
    #[allow(unused_assignments)]
    let mut content = String::new();

    loop {
        let res = client
            .get(format!(
                "https://tmi.twitch.tv/group/user/{}/chatters",
                user
            ))
            .send()
            .await;

        if let Ok(o) = res {
            let status = o.status();
            content = o.text().await.unwrap();
            if status == 200 {
                // println!("{}",content);
                let j = json::parse(&content).unwrap();
                let chatters = &j["chatters"];
                // println!("{}",chatters["broadcaster"]);
                let _broadcaster = &chatters["broadcaster"];
                let vips = &chatters["vips"];
                let moderators = &chatters["moderators"];
                let staff = &chatters["staff"];
                let admins = &chatters["admins"];
                let global_mods = &chatters["global_mods"];
                let viewers = &chatters["viewers"];

                out.append(&mut get_viewer_cathegory(vips));
                out.append(&mut get_viewer_cathegory(moderators));
                out.append(&mut get_viewer_cathegory(staff));
                out.append(&mut get_viewer_cathegory(admins));
                out.append(&mut get_viewer_cathegory(global_mods));
                out.append(&mut get_viewer_cathegory(viewers));
            } else {
                println!(
                    "get_viewers() got an error, code: {}, error:{}",
                    status, content
                )
            }
            break;
        }
    }
    //https://tmi.twitch.tv/group/user/{}/chatters

    out
}

//Requests the live status along with some stream data of specified users
//note: the requests are split up up to 100 ids
//(twitch doesn't allow more in 1 request)
pub async fn get_live_streams(ids: Vec<String>, client_id: &str, oauth: &str) -> Vec<Stream> {
    let max_size: usize = 100;
    let mut ids_tmp: Vec<String> = vec![];
    let mut ids_split: Vec<Vec<String>> = vec![];
    let mut out_streams: Vec<Stream> = vec![];

    //the acctual splitting process happens here:
    for id in ids {
        if ids_tmp.len() < max_size {
            ids_tmp.push(id);
        } else {
            ids_split.push(ids_tmp);
            ids_tmp = vec![];
        }
    }
    //push the reamaining ids
    ids_split.push(ids_tmp);

    for ids_ in ids_split {
        loop {
            let x = get_lives_from_ids(ids_.clone(), client_id, oauth).await;
            match x {
                Ok(mut o) => {
                    out_streams.append(&mut o);
                    break;
                }
                Err(e) => {
                    println!("get_lives_from_ids() error: {}\nRetrying...", e);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    }
    //sorting according to view count
    out_streams.sort_by_key(|s| s.viewer_count);
    //reversing bcs
    out_streams.reverse();
    out_streams
}

//TODO: fix this
//referece https://dev.twitch.tv/docs/api/reference#get-followed-streams
#[allow(dead_code)]
pub async fn get_live_folows(id: String, client_id: &str, oauth: &str) -> Vec<Stream> {
    let mut out_streams: Vec<Stream> = vec![];

    let client = reqwest::Client::new();
    #[allow(unused_assignments)]
    let mut res = String::new();

    let mut pagination = String::new();

    loop {
        let resp = client
            .get(format!(
                "https://api.twitch.tv/helix/streams/followed?first=100&user_id={}{}",
                id, pagination
            ))
            .header(
                reqwest::header::AUTHORIZATION,
                String::from("Bearer ") + oauth,
            )
            .header("Client-ID", client_id)
            .send()
            .await;
        println!("{:#?}", resp);
        match resp {
            #[allow(unused_assignments)]
            Ok(r) => {
                res = r.text().await.unwrap();
                let j = json::from(res);
                if j.contains("pagination") {
                    pagination = "&pagination=".to_owned() + &j["pagination"].dump();
                }

                break;
            }
            Err(e) => {
                println!("get_live_folows() got an error:{}\n retrying...", e)
            }
        }
    }
    //sorting according to view count
    out_streams.sort_by_key(|s| s.viewer_count);
    //reversing bcs
    out_streams.reverse();
    out_streams
}
