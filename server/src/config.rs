#[derive(Clone)]
pub struct Config {
    pub merkle_root: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() != 3 {
            panic!("MERKLE_ROOT env is required");
        }

        Self {
            merkle_root: args[2].clone(),
        }
    }
}
