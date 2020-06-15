// use http::{Request, Response};
use rand::seq::SliceRandom;
use reqwest::header;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, stdin, Read};
use std::path::Path;
use std::{thread, time};
use yaml_rust::YamlLoader;

static MSG_ALREADY_FRIENDS: &str = "tb_remove();window.parent.sCustomMessageString = 'You are already friends with this user.';tb_show('Error', $.ajaxBox('core.message', 'height=150&width=300'));";
static MSG_REQUEST_SENT: &str = "tb_remove();window.parent.sCustomMessageString = 'You were already requested to be friends';tb_show('Error', $.ajaxBox('core.message', 'height=150&width=300'));";
static MSG_LOGIN_REQ: &str = "tb_show('Sign in', $.ajaxBox('user.login', 'height=250&width=400'));$('body').css('cursor', 'auto');";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Starting up...");
    let file_content = fs::read_to_string("config.yaml").unwrap();
    let config_o = match YamlLoader::load_from_str(&file_content) {
        Ok(config_o) => config_o,
        Err(why) => {let cfg_err = format!("Error when reading config file: {}", why); print_and_pause(&cfg_err); panic!()},
    };
    let config = &config_o[0];
    let user_id = match config["cookies"]["user_id"].as_str() {
        Some(user_id) => user_id,
        None => {print_and_pause("Error reading from config file, download it again!"); panic!()},
    };
    let user_hash = match config["cookies"]["user_hash"].as_str() {
        Some(user_id) => user_id,
        None => {print_and_pause("Error reading from config file, download it again!"); panic!()},
    };
    let start_num = match config["start_number"].as_i64() {
        Some(user_id) => user_id,
        None => {print_and_pause("Error reading from config file, download it again!"); panic!()},
    };
    let last_known_good = match config["last_known_good"].as_i64() {
        Some(user_id) => user_id,
        None => {print_and_pause("Error reading from config file, download it again!"); panic!()},
    };
    let mut denied_ids: Vec<String> = Vec::new();
    if Path::new("denied_users.txt").exists() {
        let denied_content = match fs::File::open("denied_users.txt") {
            Ok(denied_content) => denied_content,
            Err(why) => panic!("Err: {}", why),
        };
        let denied_buffer = BufReader::new(&denied_content).lines();
        for line in denied_buffer {
            let line = match line {
                Ok(line) => line,
                Err(why) => panic!("Error: {}", why),
            };
            if line.ends_with('\n') {
                line[0..line.len() - 1].to_string();
            }
            if line.len() != 0 {
                denied_ids.push(line);
            }
        }
    }

    let header_string = format!(
        "cored9e2user_hash={}; cored9e2user_id={}",
        user_hash, user_id
    );
    let client = reqwest::Client::new();
    let sec_token = gen_sec_token();

    let mut i: i64 = start_num as i64;
    println!("Running!");

    loop {
        let uid = format!("{}", i);
        let mut core_map = HashMap::new();
        core_map.insert("core[ajax]", "true");
        core_map.insert("core[call]", "friend.addRequest");
        core_map.insert("core[security_token]", &sec_token);
        core_map.insert("core[is_admincp]", "0");
        core_map.insert("core[is_user_profile]", "1");
        core_map.insert("core[profile_user_id]", &uid);
        core_map.insert("val[user_id]", &uid);

        let mut c: i64 = 0;
        loop {
            if denied_ids.contains(&uid) {
                println!("Got to denied user id {}, skipping...", uid);
                break;
            }
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::COOKIE,
                header::HeaderValue::from_str(&header_string).unwrap(),
            );
            headers.insert(
                header::CONNECTION,
                header::HeaderValue::from_str("close").unwrap(),
            );
            let res = client
                .post("https://social.schoolrp.net/_ajax/")
                .form(&core_map)
                .headers(headers)
                .send()
                .await?;
            let returned_text = res.text().await?;

            if returned_text == MSG_REQUEST_SENT {
                println!("User {} doesn't exist or is already requested...", uid);
                c += 1;
                if c % 10 == 1 {
                    println!("Checking if person is already requested...");
                    let mut url = String::new();
                    url.push_str("https://social.schoolrp.net/profile-");
                    url.push_str(&uid);
                    let resp = reqwest::get(&url).await?;
                    if resp.status() != 404 {
                        println!("The account exists, moving on.");
                        break;
                    } else {
                        if i > last_known_good {
                            println!("The account does not yet exist...")
                        } else {
                            println!("The account is deleted, moving on...");
                            break;
                        }
                    }
                }
                thread::sleep(time::Duration::from_secs(15));
            } else if returned_text == MSG_LOGIN_REQ {
                println!("You have set the config wrong! Stop the bot and try again!");
                // input()
                panic!("Fix the config");
            } else if returned_text == MSG_ALREADY_FRIENDS {
                println!("You are already friends with {}", uid);
                break;
            } else {
                println!("Sent friend request to {}!", uid);
                break;
            }
        }
        i += 1;

        // thread::sleep(time::Duration::from_millis(100));
    }
}

fn gen_sec_token() -> String {
    let mut tbr = String::new();
    let valid_chars = vec![
        "a", "b", "c", "d", "e", "f", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
    ];
    for _ in 0..32 {
        let chare = match valid_chars.choose(&mut rand::thread_rng()) {
            Some(chare) => chare,
            None => panic!("Err!"),
        };
        tbr.push_str(chare);
    }
    return format!("{}", tbr);
}

fn print_and_pause(tex: &str) {
    println!("{}", tex);
    stdin().read(&mut []).unwrap();
}
