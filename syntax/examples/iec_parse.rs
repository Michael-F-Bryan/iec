use iec_syntax::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    let mut input = args.input().unwrap();

    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let file: File = buffer.parse().unwrap();

    match args.format {
        OutputFormat::Rust => println!("{:#?}", file),
        OutputFormat::Json => {
            serde_json::to_writer(io::stdout(), &file).unwrap()
        }
        OutputFormat::PrettyJson => {
            serde_json::to_writer_pretty(io::stdout(), &file).unwrap()
        }
    }
}

#[derive(StructOpt)]
#[structopt(
    about = "Parse some Structured Text into the corresponding Abstract Syntax Tree"
)]
struct Args {
    #[structopt(
        default_value = "-",
        parse(from_os_str),
        help = "The file to read (defaults to stdin)"
    )]
    file: PathBuf,
    #[structopt(
        short = "f",
        long = "format",
        default_value = "rust",
        raw(possible_values = "&[\"rust\", \"json\", \"pretty\"]"),
        help = "The format to use when displaying the AST"
    )]
    format: OutputFormat,
}

impl Args {
    fn input(&self) -> io::Result<Box<dyn Read>> {
        if self.file == Path::new("-") {
            Ok(Box::new(io::stdin()))
        } else {
            std::fs::File::open(&self.file)
                .map(|f| Box::new(f) as Box<dyn Read>)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    PrettyJson,
    Rust,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();

        match lowercase.as_str() {
            "json" => Ok(OutputFormat::Json),
            "pretty" => Ok(OutputFormat::PrettyJson),
            "rust" => Ok(OutputFormat::Rust),
            _ => Err("Expected a valid output format"),
        }
    }
}
