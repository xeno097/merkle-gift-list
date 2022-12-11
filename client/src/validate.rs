use clap::Parser;

#[derive(Parser, Debug)]
pub struct ValidateArgs {
    /// File that stores the list of allowed names to receive a gift
    #[arg(long, default_value = "nice-list.json")]
    file: String,

    /// Host of the server that will validate the data
    #[arg(long, default_value = "127.0.0.1:3000")]
    host: String,

    /// Name of the person to validate if it is in the list
    #[arg(long)]
    name: String,
}
