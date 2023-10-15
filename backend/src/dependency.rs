use bcrypt::{hash, verify};
use firebase_rs::Firebase;
use serde::{Serialize, Deserialize};
use settimeout::set_timeout;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub password: String, 
    pub email: String,
    pub playlists: Vec<Playlist>,
}

#[derive(Serialize, Deserialize, Debug)]  
pub struct StringResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub name: String,
    pub author: String, 
    pub cover: String,
}

pub struct Users;
impl Users {
    pub async fn create_account(firebase_client: &Firebase, user: &User) -> StringResponse {
        let _user = firebase_client.set::<User>(&user).await;
        return JSON::to_string(&_user.unwrap().data);
    }

    pub async fn delete_account(firebase_client: &Firebase, user: &String)  {
        let _user = firebase_client.at(user).delete().await.unwrap();

    }

    pub async fn get_info(firebase_client: &Firebase, userid: &String) -> User {
        let _user = firebase_client.at(&userid);
        let user = _user.get::<User>().await.unwrap();
        return user;
    }
}



pub struct JSON;
impl JSON {
    pub fn to_string(string: &str) -> StringResponse {
    	println!("{}", string);
        return serde_json::from_str(string).unwrap();
    }
}

use std::{
    io::{prelude::*, BufReader},
    net::{/*TcpListener,*/ TcpStream, Ipv4Addr, IpAddr}, fs, time::Duration, borrow::{BorrowMut, Borrow},
};
use chrono::{Utc}; 
const PATH: &str = "C:/Users//Documents/music-website/frontend";


pub struct WebServer;
impl WebServer {
    pub async fn log_in(id: String, password: String) -> bool {
        let firebase: firebase_rs::Firebase = Firebase::new(
            "https://music-site-alt-default-rtdb.firebaseio.com").unwrap().at("users");
        let info = Users::get_info(&firebase, &id).await;
        return verify(info.password, &password).unwrap();                                                                               
    }
    pub async fn handle_connection() -> Vec<(String, Firebase)> {
        println!("{} Connected", Utc::now());
        let user = User { 
            name: String::from("name"),
            password: hash(String::from("password") ,10).unwrap(), 
            email: String::from("email"),
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
        }; 
        let firebase: firebase_rs::Firebase = Firebase::new(
            "https://music-site-alt-default-rtdb.firebaseio.com").unwrap().at("users");
        let user_id = Users::create_account(&firebase, &user).await;            
        //println!("USER PASSWORD CORRECT: {}", verify(String::from("password"), user.password.as_str()).unwrap());                                                                                            
        println!("{} |  USER ACCOUNT CREATED: {} / DATA: {:?}", Utc::now(), user_id.name,
            Users::get_info(&firebase, &user_id.name).await.playlists);
        set_timeout(Duration::from_secs(1)).await; 

        let contents = format!("[ID: \"{}\"]",user_id.name.as_str()); 
        return Vec::from([(contents, firebase)]);
    }
}
