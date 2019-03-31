use iec_syntax::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    let mut input = args.input().unwrap();

    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let file: File = buffer.parse().unwrap();

    println!("{:#?}", file);
}

#[derive(StructOpt)]
struct Args {
    #[structopt(
        default_value = "-",
        parse(from_os_str),
        help = "The file to read (defaults to stdin)"
    )]
    file: PathBuf,
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
