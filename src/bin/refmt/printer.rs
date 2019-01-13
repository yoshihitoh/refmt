use std::io::Write;

use failure::ResultExt;
use syntect::easy::HighlightLines;
use syntect::util::as_24_bit_terminal_escaped;

use refmt::assets::HighlightAssets;
use refmt::errors;
use refmt::format::FormattedText;

pub trait Printer {
    fn print(&self, dest: &mut Write, text: &FormattedText) -> Result<(), errors::Error>;
}

pub struct PlainTextPrinter {}

impl Default for PlainTextPrinter {
    fn default() -> Self {
        PlainTextPrinter {}
    }
}

impl Printer for PlainTextPrinter {
    fn print(&self, dest: &mut Write, text: &FormattedText) -> Result<(), errors::Error> {
        Ok(writeln!(dest, "{}", text.text.as_str()).context(errors::ErrorKind::Io)?)
    }
}

pub struct HighlightTextPrinter<'a> {
    assets: &'a HighlightAssets,
}

impl<'a> HighlightTextPrinter<'a> {
    pub fn new(assets: &'a HighlightAssets) -> Self {
        HighlightTextPrinter { assets }
    }
}

impl<'a> Printer for HighlightTextPrinter<'a> {
    fn print(&self, dest: &mut Write, text: &FormattedText) -> Result<(), errors::Error> {
        let syntax = self.assets.get_syntax(text.format.preferred_extension());
        let theme = self.assets.get_default_theme();
        let mut highlight = HighlightLines::new(syntax, theme);
        let ranges = highlight.highlight(&text.text, &self.assets.syntax_set);
        let escaped = as_24_bit_terminal_escaped(&ranges, true);
        Ok(writeln!(dest, "{}", escaped).context(errors::ErrorKind::Io)?)
    }
}
