use std::fmt;
use std::fmt::{Display, Formatter};
use clap::{Parser, Args, Subcommand, ValueEnum};

/// Brief description of what this application does.
#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None, arg_required_else_help(true))]
pub struct Cli {

    #[arg(short, long)]
    pub build: bool,

    /// Specify alternate output format
    #[arg(short, long, requires("build"))]
    pub output_format: Option<OutputFormat>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    /// json formatted text
    Json,

    /// yaml formatted text
    Yaml
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Gen(Generate),
    Test(Fd),
}

#[derive(Args, Debug)]
pub struct Fd {
    /// CN for root ca
    #[arg(short, long, default_value("Root ca"))]
    one: String,
    /// CN for signing ca
    #[arg(short, long, default_value("Intermediate Signing ca"))]
    two: String,
}

#[derive(Args, Debug)]
pub struct Generate {
    /// CN for root ca
    #[arg(short, long, default_value("Root ca"))]
    pub root_cn: String,
    /// CN for signing ca
    #[arg(short, long, default_value("Intermediate Signing ca"))]
    pub signing_cn: String,

    /// Generate expired client and server certificates
    #[arg(short, long)]
    pub expired: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
