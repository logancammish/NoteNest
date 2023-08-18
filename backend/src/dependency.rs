use firebase_rs::{ * };
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


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

pub struct JSON;
impl JSON {
    pub fn to_string(string: &str) -> StringResponse {
        return serde_json::from_str(string).expect("There was a problem 
                                            converting string data to a JSON objecet");
    }
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