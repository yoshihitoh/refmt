use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

const DEFAULT_THEME: &str = "Monokai Extended";

pub struct HighlightAssets {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl HighlightAssets {
    pub fn new(syntax_set: SyntaxSet, theme_set: ThemeSet) -> HighlightAssets {
        HighlightAssets {
            syntax_set,
            theme_set,
        }
    }

    pub fn get_syntax(&self, name: &str) -> &SyntaxReference {
        self.syntax_set.find_syntax_by_extension(name).unwrap()
    }

    pub fn syntaxes(&self) -> &[SyntaxReference] {
        self.syntax_set.syntaxes()
    }

    pub fn get_default_theme(&self) -> &Theme {
        self.get_theme(DEFAULT_THEME)
    }

    pub fn get_theme(&self, name: &str) -> &Theme {
        &self.theme_set.themes[name]
    }

    pub fn themes(&self) -> Vec<&Theme> {
        self.theme_set.themes.iter().map(|(_, v)| v).collect()
    }
}
