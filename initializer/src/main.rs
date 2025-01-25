use actix_web::{App, HttpServer};
mod handler;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = 3030;

    let server = HttpServer::new(move || App::new().configure(handler::routes))
        .bind(("0.0.0.0", 3030))
        .unwrap_or_else(|_| panic!("Can not bind to {}", &port))
        .run();

    log::info!("generator-client started on port {}", port);

    server.await
}

#[cfg(test)]
mod tests {
    use simplecrypt;
    use crate::handler::Config;

    #[test]
    fn test_encrypt_decrypt() {
        let data = "Some random data";
        let key = "BlahBlah123";

        let encrypted = simplecrypt::encrypt(data.as_bytes(), key.as_bytes());

        let sample_config = Config {
            xai_api_key: encrypted.clone(),
            encryption_key: key.as_bytes().to_vec()
        };

        println!("Sample data: {:?}", sample_config);

        let decrypted = simplecrypt::decrypt(&encrypted, key.as_bytes()).unwrap();
        assert_eq!(data.as_bytes().to_vec(), decrypted);
    }
}