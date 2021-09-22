use actix_web::{Responder, HttpResponse, App, test};
use actix_web_codegen::*;

#[get("/config")]
async fn config() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() {
    let srv = test::start(|| App::new().service(config));

    let request = srv.get("/config");
    let response = request.send().await.unwrap();
    assert!(response.status().is_success());
}
