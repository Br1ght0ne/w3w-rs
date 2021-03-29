use std::{
    fs,
    io::{self, BufRead, Read},
    path,
};

use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};
use tracing_subscriber::prelude::*;

use w3w_api::Client;

/// CLI for what3words public API.
#[derive(Debug, StructOpt)]
struct App {
    /// Your what3words API key.
    #[structopt(long, short = "k", env = "W3W_API_KEY", hide_env_values = true)]
    api_key: String,
    /// Output format to write to stdout
    #[structopt(long, short, default_value, value_name = "format", possible_values = OutputFormat::VARIANTS, env = "W3W_OUTPUT_FORMAT")]
    output_format: OutputFormat,
    /// Log format to be used by tracing-subscriber
    #[structopt(long, default_value, value_name = "format", possible_values = LogFormat::VARIANTS, env = "W3W_LOG_FORMAT")]
    log_format: LogFormat,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// List all available languages for three-word-addresses.
    AvailableLanguages,
    /// Convert three-word-addresses to geographic coordinates.
    ToCoords {
        /// File to read three-word-addresses from.
        file: Option<path::PathBuf>,
    },
    /// Convert geographic coordinates to three-word-addresses.
    #[structopt(name = "to-3wa")]
    To3wa {
        /// File to read coordinates from.
        file: Option<path::PathBuf>,
    },
}

#[derive(Debug, strum::Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "camelCase")]
enum OutputFormat {
    Plain,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Plain
    }
}

#[derive(Debug, strum::Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "camelCase")]
enum LogFormat {
    Compact,
    Full,
    Json,
    Pretty,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Full
    }
}

fn main() -> anyhow::Result<()> {
    let app = App::from_args();
    setup_logging(&app.log_format)?;
    run(app)
}

fn setup_logging(format: &LogFormat) -> anyhow::Result<()> {
    let filter_layer =
        tracing_subscriber::EnvFilter::from_default_env().add_directive("ureq=warn".parse()?);
    let builder = tracing_subscriber::registry().with(filter_layer);
    match format {
        LogFormat::Compact => builder
            .with(tracing_subscriber::fmt::layer().compact())
            .init(),
        LogFormat::Full => builder.with(tracing_subscriber::fmt::layer()).init(),
        LogFormat::Json => builder.with(tracing_subscriber::fmt::layer().json()).init(),
        LogFormat::Pretty => builder
            .with(tracing_subscriber::fmt::layer().pretty())
            .init(),
    };
    Ok(())
}

fn run(app: App) -> anyhow::Result<()> {
    tracing::info!("starting w3w-cli");
    let client = Client::new(&app.api_key);

    match &app.command {
        Command::AvailableLanguages => {
            let languages = client.available_languages()?;
            print(&languages, &app.output_format)?;
        }
        Command::ToCoords { file } | Command::To3wa { file } => {
            let input: Box<dyn Read> = match file {
                Some(path) if path.to_string_lossy() != "-" => {
                    tracing::info!(path = %path.display(), "reading from file");
                    Box::new(fs::File::open(path)?)
                }
                _ => {
                    tracing::info!("reading from stdin");
                    Box::new(io::stdin())
                }
            };
            let input = io::BufReader::new(input);

            for line in input.lines() {
                let line = line?;
                let coords = client.convert_to_coordinates(&line)?;
                tracing::trace!(?line, %coords);
                print(&coords, &app.output_format)?;
            }
        }
    }
    tracing::info!("success, exiting");
    Ok(())
}

fn print<T>(value: &T, format: &OutputFormat) -> anyhow::Result<()>
where
    T: ToString + serde::Serialize,
{
    println!(
        "{}",
        match format {
            OutputFormat::Plain => value.to_string(),
            OutputFormat::Json => serde_json::to_string(value)?,
        }
    );
    Ok(())
}
