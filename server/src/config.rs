use std::env;

#[derive(Clone, Default)]
pub struct Config {
    pub port: i32,
    pub merkle_root: String,
}

impl Config {
    pub fn new() -> Self {
        let merkle_root = env::var("MERKLE_ROOT").expect("MERKLE_ROOT env variable is required");
        let port: i32 = env::var("PORT")
            .unwrap_or_else(|_| 3000.to_string())
            .parse()
            .expect("PORT env variable should be a number");

        Self { port, merkle_root }
    }
}
