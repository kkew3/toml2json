#![forbid(unsafe_code)]

use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

enum Error {
    Io,
    Deser,
    Ser,
}

macro_rules! impl_from_err {
    ($err_type:ty, $variant:path) => {
        impl From<$err_type> for Error {
            fn from(_err: $err_type) -> Self {
                $variant
            }
        }
    };
}

impl_from_err!(io::Error, Self::Io);
impl_from_err!(toml::de::Error, Self::Deser);
impl_from_err!(serde_json::Error, Self::Ser);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io => write!(f, "E: failed to collect stdin"),
            Self::Ser => write!(f, "E: failed to collect from stdin"),
            Self::Deser => write!(
                f,
                "E: JSON serialization and/or stdout streaming failed"
            ),
        }
    }
}

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// The TOML to convert [default: stdin]
    input: Option<PathBuf>,
}

fn app(cli: Cli) -> Result<(), Error> {
    // Get our input source from stdin. We don't bother streaming or chunking,
    // since the `toml` crate only supports slices and strings.
    let mut input_buf = Vec::new();
    read_input(cli.input, &mut input_buf)?;

    // Turn our collected input into a value. We can't be more specific than
    // value, since we're doing arbitrary valid TOML conversions.
    let value: toml::Value = toml::from_slice(&input_buf)?;

    // Spit back out, but as JSON. `serde_json` *does* support streaming, so
    // we do it.
    serde_json::to_writer(io::stdout(), &value)?;

    Ok(())
}

fn read_input(input: Option<PathBuf>, buf: &mut Vec<u8>) -> io::Result<()> {
    match input {
        None => io::stdin().read_to_end(buf)?,
        Some(path) => File::open(path)?.read_to_end(buf)?,
    };
    Ok(())
}

fn eprintln_err_and_exit(err: Error) -> ExitCode {
    eprintln!("{}", err);
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    app(cli)
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(eprintln_err_and_exit)
}
