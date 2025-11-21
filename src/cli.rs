use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Send a chat message to the Ollama model
    Chat {
        /// The chat message in t3 format (JSON string)
        #[clap(value_parser)]
        message: String,
    },
}
