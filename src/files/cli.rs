use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "futils")]
#[command(about = "A CLI to search files like ls")]
pub struct Cli {
    #[arg(default_value = "./")]
    pub path: String,

    #[arg(short, long, default_value = ".*")]
    pub regex: String,

    #[arg(short, long, default_value = "")]
    pub search: String,

    #[arg(short, long, default_value = "*")]
    pub file_type: String,

    #[arg(long, default_value = "")]
    pub add: String,
}
