use firebase_rs::{ * };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    password: String, 
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]  
struct StringResponse {
    name: String,
}

struct JSON;
impl JSON {
    fn to_string(string: &str) -> StringResponse {
        return serde_json::from_str(string).expect("There was a problem 
                                            converting string data to a JSON objecet");
    }
}

struct Users;
impl Users {
    async fn create_account(firebase_client: &Firebase, user: &User) -> StringResponse {
        let _user = firebase_client.at("users").set::<User>(&user).await;
        return JSON::to_string(&_user.unwrap().data);
    }

    async fn delete_accout(firebase_client: &Firebase, user: &String) -> &str {
        let _user = firebase_client.at("users").delete(&user).await;
        return JSON::to_string(&_user.unwrap().data);
    }
}


#[tokio::main] 
async fn main() {
    let user = User { 
        name: String::from("one"),
        password: String::from("two"),
        email: String::from("three")
     };
    let firebase: firebase_rs::Firebase = Firebase::new("").unwrap().at("two");
    println!("{}", set_user(&firebase, &user).await.name);
}

