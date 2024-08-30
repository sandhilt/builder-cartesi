use std::error::Error;

use builder_cartesi::http_service;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    match http_service::create_server() {
        Ok(server) => {
            server.await?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error creating server: {}", err);
            Err(err)
        }
    }
}
