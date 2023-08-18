mod dependency;
use dependency::{ * };
use firebase_rs::{ * };
use bcrypt::{hash, verify};
use std::{time::Duration, collections::HashMap};
use settimeout::set_timeout;

#[tokio::main] 
async fn main() {
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
        ])
    }; // user account information
    let firebase: firebase_rs::Firebase = Firebase::new(
        "https://.firebaseio.com").unwrap().at("users"); // open users directory in
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
    println!("Deleted account: {}", user_id.name);
}

