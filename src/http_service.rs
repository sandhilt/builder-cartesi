use std::{error::Error, sync::Mutex};

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

struct AppState {
    counter: Mutex<u64>,
}

#[derive(Debug, Deserialize)]
struct Dapp {
    name: String,
    version: String,
    description: Option<String>,
    language: String,
    dependencies: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct QueryLanguage {
    language: String,
}

#[derive(Serialize)]
struct ResponseLanguageFeatures {
    features: Vec<String>,
}

#[derive(Serialize)]
struct CountResponse {
    app_count: u64,
}

#[derive(Deserialize)]
struct DappResources {}

pub fn create_server() -> Result<Server, Box<dyn Error>> {
    let server = HttpServer::new(|| {
        let state = web::Data::new(AppState {
            counter: Mutex::new(0),
        });
        App::new()
            .app_data(state)
            .service(hello)
            .service(count)
            .service(create_app)
    })
    .bind(("127.0.0.1", 8076))?
    .run();

    Ok(server)
}

#[get("/language/{language}")]
async fn get_language(data: web::Path<QueryLanguage>) -> impl Responder {
    let language = data.language.to_lowercase();

    let response = ResponseLanguageFeatures { features: vec![] };

    match language.as_str() {
        "c++" => HttpResponse::Ok().json(response),
        "go" => HttpResponse::Ok().json(response),
        "javascript" => HttpResponse::Ok().json(response),
        "lua" => HttpResponse::Ok().json(response),
        "python" => HttpResponse::Ok().json(response),
        "ruby" => HttpResponse::Ok().json(response),
        "rust" => HttpResponse::Ok().json(response),
        "typescript" => HttpResponse::Ok().json(response),
        _ => HttpResponse::NotFound().body("Unsupported language"),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/count")]
async fn count(state: web::Data<AppState>) -> impl Responder {
    let counter = state.counter.lock().unwrap();
    let response = CountResponse {
        app_count: *counter,
    };
    HttpResponse::Ok().json(response)
}

#[post("/create_app")]
async fn create_app(state: web::Data<AppState>, request: web::Json<Dapp>) -> impl Responder {
    println!("Creating app");
    let mut counter = state.counter.lock().unwrap();
    println!("App request {:?}", request);
    *counter += 1;
    println!("App {} create", (*counter));
    HttpResponse::NoContent().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::with_uri("/").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_count() {
        let state = web::Data::new(AppState {
            counter: Mutex::new(0),
        });
        let app = test::init_service(App::new().app_data(state).service(count)).await;
        let req = test::TestRequest::with_uri("/count").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success(), "Response: {:?}", res);
        let body_expected = json!({ "app_count": 0 });
        assert_eq!(test::read_body(res).await, body_expected.to_string());
    }

    #[actix_web::test]
    async fn test_get_language() {
        let app = test::init_service(App::new().service(get_language)).await;
        let req = test::TestRequest::with_uri("/language/rust").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success(), "Response: {:?}", res);
        let body_received = test::read_body(res).await;
        let body_value: serde_json::Value = serde_json::from_slice(&body_received).unwrap();
        assert!(body_value.get("features").unwrap().is_array(), "Body: {:?}", body_received);
    }

    #[actix_web::test]
    async fn test_create_app() {
        let state = web::Data::new(AppState {
            counter: Mutex::new(0),
        });
        let app = test::init_service(App::new().app_data(state).service(create_app)).await;
        let req_body = json!({
            "name": "myapp",
            "version": "0.1.0",
            "language": "rust"
        });
        let req = test::TestRequest::post()
            .set_json(req_body)
            .uri("/create_app")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success(), "Response: {:?}", res);
    }
}
