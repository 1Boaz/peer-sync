use clap_derive::Parser;

#[derive(Parser, Debug)]
pub struct ReceiverArgs {
    /// Port to listen on
    /// Default: 8080
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
}