use actix_web::{post, web::Data, App, HttpResponse, HttpServer, Responder};

use crate::config::Config;

#[post("/gift")]
async fn gift() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run_server(config: Config) -> std::io::Result<()> {
    HttpServer::new(move || App::new().app_data(Data::new(config.clone())).service(gift))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
