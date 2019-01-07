use std::path::Path;

use syntect::dumps::{dump_to_file, from_binary};
use syntect::highlighting::ThemeSet;
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};

struct AssetBuilder<'a> {
    asset_dir: &'a Path,
}

impl<'a> AssetBuilder<'a> {
    fn build_syntaxes(&self) -> Result<SyntaxSet, syntect::LoadingError> {
        let mut syntax_set_builder = SyntaxSetBuilder::new();
        syntax_set_builder.add_plain_text_syntax();
        syntax_set_builder.add_from_folder(self.asset_dir.join("syntaxes"), true)?;

        Ok(syntax_set_builder.build())
    }

    fn build_themes(&self) -> Result<ThemeSet, syntect::LoadingError> {
        let mut theme_set = ThemeSet::default();
        theme_set.add_from_folder(self.asset_dir.join("themes"))?;

        Ok(theme_set)
    }
}

struct AssetLoader {}

impl AssetLoader {
    fn load_syntaxes(&self) -> SyntaxSet {
        from_binary(include_bytes!("../assets/syntaxes.bin"))
    }

    fn load_themes(&self) -> ThemeSet {
        from_binary(include_bytes!("../assets/themes.bin"))
    }
}

impl Default for AssetLoader {
    fn default() -> Self {
        AssetLoader {}
    }
}

pub struct HighlightAssets {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl HighlightAssets {
    pub fn load_integrated() -> Result<HighlightAssets, syntect::LoadingError> {
        let loader = AssetLoader::default();
        Ok(HighlightAssets {
            syntax_set: loader.load_syntaxes(),
            theme_set: loader.load_themes(),
        })
    }

    pub fn build_from_files(asset_dir: &Path) -> Result<HighlightAssets, syntect::LoadingError> {
        let builder = AssetBuilder { asset_dir };
        Ok(HighlightAssets {
            syntax_set: builder.build_syntaxes()?,
            theme_set: builder.build_themes()?,
        })
    }

    pub fn save(&self, asset_dir: &Path) -> Result<(), bincode::Error> {
        // TODO: Remove bincode from dependencies. It is only used for this error handling.
        dump_to_file(&self.syntax_set, asset_dir.join("syntaxes.bin"))?;
        dump_to_file(&self.theme_set, asset_dir.join("themes.bin"))?;

        Ok(())
    }
}
