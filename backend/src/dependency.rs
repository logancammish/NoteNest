use bcrypt::{hash, verify};
use firebase_rs::Firebase;
use serde::{Serialize, Deserialize};
use settimeout::set_timeout;

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
        return serde_json::from_str(string).expect("There was a problem 
                                            converting string data to a JSON objecet");
    }
}

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, Ipv4Addr, IpAddr}, fs, time::Duration, borrow::{BorrowMut, Borrow},
};
use chrono::{Utc}; 
const PATH: &str = "C:/Users//Documents/music-website/frontend";

pub struct WebServer;
impl WebServer {
    pub async fn establish() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let c_stream = stream.unwrap();
            println!("{} : Connection established", Utc::now());

            WebServer::handle_connection(c_stream).await;
        }
    }

    async fn handle_connection(mut stream: TcpStream) {
        let ip = stream.local_addr().unwrap().ip();
        println!("{} | {:?} Connected", Utc::now(), ip);

        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        if request_line == "GET / HTTP/1.1" {            
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
                "https://x.firebaseio.com").unwrap().at("users");
            let user_id = Users::create_account(&firebase, &user).await;            
            //println!("USER PASSWORD CORRECT: {}", verify(String::from("password"), user.password.as_str()).unwrap());                                                                                            
            println!("{} | {} / USER ACCOUNT CREATED: {} / DATA: {:?}", Utc::now(), ip, user_id.name,
                Users::get_info(&firebase, &user_id.name).await.playlists);
            set_timeout(Duration::from_secs(1)).await; 

            let contents = format!("[ID: \"{}\"]",user_id.name.as_str()); 
            let length = contents.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        } 
    }
}