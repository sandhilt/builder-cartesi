use std::error::Error;

use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

pub fn create_server() -> Result<Server, Box<dyn Error>> {
    let server = HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8076))?
        .run();

    Ok(server)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::with_uri("/").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }
}
