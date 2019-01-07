use std::path::Path;

use clap::{App, Arg};
use reser_core::assets::HighlightAssets;

#[derive(Debug)]
struct ProgramOption {
    assert_dir: String,
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
            Arg::with_name("ASSET_DIR")
                .long("asset-dir")
                .help("Set path to asset directory")
                .takes_value(true)
                .value_name("ASSET_DIR")
                .required(true),
        )
        .get_matches();

    ProgramOption {
        assert_dir: m.value_of("ASSET_DIR").unwrap().to_string(),
    }
}

fn main() -> Result<(), GenerateError> {
    let options = parse_args();
    let assets = HighlightAssets::build_from_files(Path::new(&options.assert_dir))?;
    assets.save(Path::new(&options.assert_dir))?;
    Ok(())
}
