use std::{future::{Future, Ready}, pin::Pin};

use firebase_rs::{*};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: [String; 3],
    password: String, 
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]  
struct Response {
    name: String,
}


async fn set_user(firebase: &firebase_rs::Firebase, user: &User) -> Response {
    let _user = firebase.set::<User>(&user).await;
    return string_to_response(String::from(&_user.unwrap().data)).await;
}

async fn string_to_response(string: String) -> Response {
   serde_json::from_str::<Response>(string.as_str()).unwrap()
}

#[tokio::main] 
async fn main() {
    let user = User { 
        name: [
            String::from("One"),
            String::from("Two"),
            String::from("Three")
            ],
        password: String::from("Four"),
        email: String::from("Five")
     };
    let firebase: firebase_rs::Firebase = Firebase::auth("https://music-site-alt.firebaseapp.com", "*").unwrap().at("two");
    println!("{}",set_user(&firebase, &user).await.name);
}

