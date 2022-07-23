use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[clap(name = "qrrs", author, about, version)]
pub struct Arguments {
    /// Input data
    #[clap(
        name = "INPUT",
        value_hint = ValueHint::AnyPath,
        required(true),
        index(1)
    )]
    pub input: Option<String>,

    /// Output file
    #[clap(
        name = "OUTPUT",
        value_hint = ValueHint::AnyPath,
        required_unless_present_any(["INPUT", "read", "terminal"]),
        index(2)
    )]
    pub output: Option<String>,

    /// Reads the qr-code instead of generating it
    #[clap(name = "read", short, long)]
    pub read: bool,

    /// Displays code in terminal
    #[clap(name = "terminal", short, long)]
    pub terminal_output: bool,
}
