use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Charset {
    Block,
    Hash,
    Dot,
}

#[derive(Parser, Debug)]
#[command(name = "rsqr")]
#[command(version = "0.3.0")]
#[command(about = "Generate QR codes in your terminal or as PNG")]
pub struct Args {
    #[arg(conflicts_with = "me")]
    pub text: Option<String>,

    #[arg(long)]
    pub me: bool,

    #[arg(long)]
    pub invert: bool,

    #[arg(long)]
    pub no_quiet: bool,

    #[arg(long, value_enum, default_value_t = Charset::Block)]
    pub charset: Charset,

    #[arg(long)]
    pub png: Option<String>,

    #[arg(long, default_value_t = 10)]
    pub scale: u32,
}
