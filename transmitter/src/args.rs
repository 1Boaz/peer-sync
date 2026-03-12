use clap::Parser;


#[derive(Debug, Parser)]
pub struct TransmitterArgs {
    #[arg(long, short, required = true)]
    pub ip: String,

    #[arg(long, short, default_value_t = 31415)]
    /// Port to host the receiver on
    pub port: u16,

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    /// Files to transfer, separated by <space>
    pub files: Vec<String>
}