mod dependency;
use dependency::{ * };
use firebase_rs::{ * };
use bcrypt::{hash, verify};
use std::time::Duration;
use settimeout::set_timeout;

#[tokio::main] 
async fn main() {
    WebServer::establish().await;
    
/*
    let user = User { 
        name: String::from("name"),
        password: hash(String::from("password") ,10).unwrap(), // use bcrypt to hash the password
        email: String::from("name"),
        playlists: Vec::from([
            Playlist {
                name: "Copyright Free".to_string(),
                author: "name".to_string(),
                cover: "random".to_string(),
            },
            Playlist {
                name: "fusdijkof".to_string(),
                author: "sjdfks".to_string(),
                cover: "random".to_string()
            }
        ])
    }; // user accoun6t information
    let firebase: firebase_rs::Firebase = Firebase::new(
        "https://music-site-alt-default-rtdb.firebaseio.com").unwrap().at("users"); // open users directory in
                                                                                  // realtime database
    let user_id = Users::create_account(&firebase, &user).await; // create user account and take
                                                                                 // stored id 
    
    println!("{}", verify(String::from("password"), user.password.as_str()).unwrap()); // verify password versus
                                                                                    // encrypted version
                                                                                      

    println!("User id: {}", user_id.name);
    println!("{}",&user_id.name);
    println!("Info: {:?}", Users::get_info(&firebase, &user_id.name).await.playlists);
    set_timeout(Duration::from_secs(5)).await; // wait 5 seconds
    Users::delete_account(&firebase, &user_id.name).await; // delete the account
    println!("Deleted account: {}", user_id.name);*/
}

