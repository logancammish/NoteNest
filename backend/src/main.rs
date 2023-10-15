mod dependency;
use dependency::{ * };
use firebase_rs::{ * };
use bcrypt::{hash, verify};
use std::time::Duration;
use settimeout::set_timeout;

use poem::{listener::TcpListener, Route, Server, middleware::Cors};
use poem_openapi::{param::Query, payload::PlainText, payload::Response, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/make_account", method = "get")]
    async fn run(&self) -> PlainText<String> {
        let info = WebServer::handle_connection().await;
        let info = info[0].0.clone();
        return PlainText(info);
    }

    #[oai(path = "/login", method = "get")]
    async fn index(&self, id: Query<Option<String>>, password: Query<Option<String>>) -> PlainText<String>  {
        println!("working so far");
        let id = id.0.unwrap();
        let password = password.0.unwrap();
        let info = WebServer::log_in(id, password).await;
        return PlainText(info.to_string());
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "notenest api", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    Server::new(TcpListener::bind("127.0.0.1:3300"))
        .run(poem::EndpointExt::with(Route::new().nest("/api",api_service).nest("/",ui), Cors::new()))
        .await
}

/*
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

*/
