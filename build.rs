use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum, CommandFactory};
use clap_complete::{generate_to, shells::{Bash, Zsh, Fish}};


// ✅ Inline copy of the CLI options for build script only
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Charset {
    Block,
    Hash,
    Dot,
}

#[derive(Parser, Debug)]
#[command(name = "rsqr")]
#[command(version = "1.0.0")]
#[command(about = "Generate QR codes in your terminal or as PNG")]
struct Args {
    #[arg(conflicts_with = "me")]
    text: Option<String>,

    #[arg(long)]
    me: bool,

    #[arg(long)]
    invert: bool,

    #[arg(long)]
    no_quiet: bool,

    #[arg(long, value_enum, default_value_t = Charset::Block)]
    charset: Charset,

    #[arg(long)]
    png: Option<String>,

    #[arg(long, default_value_t = 10)]
    scale: u32,
}

fn main() {
    let out_dir = PathBuf::from("completions");
    fs::create_dir_all(&out_dir).expect("failed to create completions dir");

    let mut cmd = Args::command();

    generate_to(Bash, &mut cmd, "rsqr", &out_dir).unwrap();
    generate_to(Zsh, &mut cmd, "rsqr", &out_dir).unwrap();
    generate_to(Fish, &mut cmd, "rsqr", &out_dir).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
