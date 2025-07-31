use clap::ValueEnum;

/// Character rendering styles for terminal output
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Charset {
    Block,
    Hash,
    Dot,
}
