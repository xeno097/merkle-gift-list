use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use merkle_tree::MerkleTreeProofNode;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Deserialize, Debug)]
struct GiftValidationRequest {
    name: String,
    proof: Vec<MerkleTreeProofNode>,
}

#[derive(Serialize)]
struct GiftValidationResponse {
    result: String,
}

impl Responder for GiftValidationResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let parsed_body = serde_json::to_string(&self);

        match parsed_body {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[post("/gift")]
async fn gift(config: Data<Config>, data: Json<GiftValidationRequest>) -> impl Responder {
    let result =
        if merkle_tree::validate(data.name.clone(), config.merkle_root.clone(), &data.proof) {
            String::from("You got a toy robot!")
        } else {
            String::from("You are not on the list :(")
        };

    GiftValidationResponse { result }
}

pub async fn run_server() -> std::io::Result<()> {
    let config = Config::new();

    HttpServer::new(move || App::new().app_data(Data::new(config.clone())).service(gift))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
