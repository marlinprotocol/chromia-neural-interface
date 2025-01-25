use actix_web::{get, http::StatusCode, post, web, Responder};
use std::env;
use simplecrypt::decrypt;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use kalypso_helper::response::response;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub xai_api_key: Vec<u8>,
    pub encryption_key: Vec<u8>,
}

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    response(
        "Hello, the initializer is up!",
        StatusCode::OK,
        Some("Hello, the initializer is up!".into()),
    )
}

#[get("/get_key")]
async fn get_key() -> impl Responder {
    // Fetch key
    dotenv().ok();
    let _api_key: String = env::var("XAI_API_KEY")
        .unwrap_or_else(|_| panic!("XAI_API_KEY must be provided in the .env file"))
        .parse::<String>()
        .expect("XAI_API_KEY must be valid");

    response(
        "Api key fetched.",
        StatusCode::OK,
        None,
    )
}

#[post("/set_key")]
async fn set_key(
    payload: web::Json<Config>,
) -> impl Responder {
    // Check if key already exixts
    dotenv().ok();
    let api_key_get = env::var("XAI_API_KEY");

    match api_key_get {
        Ok(_) => {
            panic!("XAI_API_KEY key is already set");
        }
        Err(_) => { }
    }

    // Decrypt the API key & store the values to .env file
    let key_encrypted = payload.clone().xai_api_key;
    let encryption_key = payload.clone().encryption_key;
    let api_key = decrypt(&key_encrypted, &encryption_key).unwrap();

    let key_set = Config {
        xai_api_key: key_encrypted,
        encryption_key
    };

    let api_key_str = String::from_utf8(api_key).unwrap();

    let env_key = "XAI_API_KEY";
    unsafe {
        env::set_var(env_key, api_key_str);
    }

    response(
        "Api key set.",
        StatusCode::OK,
        Some(Value::String(serde_json::to_string(&key_set).unwrap())),
    )

}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(set_key)
        .service(get_key);
    conf.service(scope);
}
