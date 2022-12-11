use std::env;

use server::{config::Config, server::run_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_args: Vec<String> = env::args().collect();

    let app_config = Config::new(&env_args);

    run_server(app_config).await
}
