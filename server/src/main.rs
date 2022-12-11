use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/gift")]
async fn gift() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(gift))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
