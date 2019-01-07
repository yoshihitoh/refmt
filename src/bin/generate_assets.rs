use std::path::Path;

use clap::{App, Arg};
use refmt_core::assets::HighlightAssets;

#[derive(Debug)]
struct ProgramOption {
    assets_dir: String,
}

#[derive(Debug)]
enum GenerateError {
    SyntectError(syntect::LoadingError),
    BincodeError(bincode::Error),
}

impl From<syntect::LoadingError> for GenerateError {
    fn from(e: syntect::LoadingError) -> Self {
        GenerateError::SyntectError(e)
    }
}

impl From<bincode::Error> for GenerateError {
    fn from(e: bincode::Error) -> Self {
        GenerateError::BincodeError(e)
    }
}

fn parse_args() -> ProgramOption {
    let m = App::new("generate_assets")
        .arg(
            Arg::with_name("ASSETS_DIR")
                .long("assets-dir")
                .help("Set path to assets directory")
                .takes_value(true)
                .value_name("ASSETS_DIR")
                .required(true),
        )
        .get_matches();

    ProgramOption {
        assets_dir: m.value_of("ASSETS_DIR").unwrap().to_string(),
    }
}

fn main() -> Result<(), GenerateError> {
    let options = parse_args();
    let assets = HighlightAssets::build_from_files(Path::new(&options.assets_dir))?;
    assets.save(Path::new(&options.assets_dir))?;
    Ok(())
}
