use std::path::Path;

use anyhow::{anyhow, Context};
use clap::{App, Arg};
use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};

use refmt::assets::HighlightAssets;

struct AssetBuilder {}

impl AssetBuilder {
    pub fn build_from_files(
        syntaxes_dir: &Path,
        themes_dir: &Path,
    ) -> Result<HighlightAssets, anyhow::Error> {
        let builder = AssetBuilder::default();
        Ok(HighlightAssets {
            syntax_set: builder.build_syntaxes(syntaxes_dir)?,
            theme_set: builder.build_themes(themes_dir)?,
        })
    }

    pub fn save(assets: HighlightAssets, assets_dir: &Path) -> Result<(), anyhow::Error> {
        dump_to_file(&assets.syntax_set, assets_dir.join("syntaxes.bin"))
            .context("Cannot create assets sytaxes.bin")?;
        dump_to_file(&assets.theme_set, assets_dir.join("themes.bin"))
            .context("Cannot create assets theme.bin")?;

        Ok(())
    }

    fn build_syntaxes(&self, syntaxes_dir: &Path) -> Result<SyntaxSet, anyhow::Error> {
        let mut syntax_set_builder = SyntaxSetBuilder::new();
        syntax_set_builder.add_plain_text_syntax();
        syntax_set_builder
            .add_from_folder(syntaxes_dir, true)
            .map_err(|e| anyhow!("Cannot create syntaxes. error:{:?}", e))?;

        Ok(syntax_set_builder.build())
    }

    fn build_themes(&self, themes_dir: &Path) -> Result<ThemeSet, anyhow::Error> {
        let mut theme_set = ThemeSet::default();
        theme_set
            .add_from_folder(themes_dir)
            .map_err(|e| anyhow!("Cannot create themes. error:{:?}", e))?;

        Ok(theme_set)
    }
}

impl Default for AssetBuilder {
    fn default() -> Self {
        AssetBuilder {}
    }
}

#[derive(Debug)]
struct ProgramOption {
    assets_dir: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = parse_args();
    let assets_dir = Path::new(&options.assets_dir);
    let assets =
        AssetBuilder::build_from_files(&assets_dir.join("syntaxes"), &assets_dir.join("themes"))?;
    AssetBuilder::save(assets, Path::new(&options.assets_dir))?;
    Ok(())
}
