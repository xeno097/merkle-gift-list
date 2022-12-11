use clap::Parser;

/// Client to validate if a person is in the Gift List
#[derive(Parser, Debug)]
struct Args {
    /// Host of the server that will validate the data
    #[arg(long, default_value = "127.0.0.1:3000")]
    host: String,

    /// File that stores the allowed names to receive a gift
    #[arg(long, default_value = "nice-list.json")]
    file: String,

    /// Name of the person to validate if it is in the list
    #[arg(long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args)
}
