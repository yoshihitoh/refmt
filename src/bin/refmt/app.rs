use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::str::FromStr;

use clap::{crate_authors, crate_name, crate_version, App as ClapApp, AppSettings, Arg};
use log::debug;
use syntect::dumps::from_binary;

use refmt::assets::HighlightAssets;
use refmt::errors;
use refmt::format::{FileFormat, FormattedText};

use crate::printer::{HighlightTextPrinter, PlainTextPrinter, Printer};

#[derive(Debug)]
struct Config {
    input_file: Option<String>,
    input_format: FileFormat,
    output_file: Option<String>,
    output_format: FileFormat,
    color_enabled: bool,
    theme_name: String,
}

fn infer_format_name<'a>(file: Option<&'a str>, format_name: Option<&'a str>) -> Option<&'a str> {
    if let Some(format_name) = format_name {
        Some(format_name)
    } else if let Some(file) = file {
        Path::new(file).extension().and_then(|ext| ext.to_str())
    } else {
        None
    }
}

fn infer_format(
    file: Option<&str>,
    format_name: Option<&str>,
) -> Result<FileFormat, errors::Error> {
    let format_name = infer_format_name(file, format_name);
    if let Some(format_name) = format_name {
        FileFormat::from_str(format_name)
    } else {
        Err(errors::Error::InferFormat)
    }
}

impl Config {
    fn new(app: ClapApp, color_enabled: bool) -> Result<Self, errors::Error> {
        let matches = app.get_matches();
        let input_file = matches.value_of("INPUT_FILE");
        debug!("input_file: {:?}", input_file);

        let input_format = matches.value_of("INPUT_FORMAT");
        debug!("input_format: {:?}", input_format);
        let input_format = infer_format(input_file, input_format)?;

        let output_file = matches.value_of("OUTPUT_FILE");
        debug!("output_file: {:?}", output_file);

        let output_format = matches.value_of("OUTPUT_FORMAT");
        debug!("output_format: {:?}", output_format);
        let output_format = infer_format_name(output_file, output_format)
            .map(FileFormat::from_str)
            .unwrap_or_else(|| Ok(input_format))?;

        let theme_name = "Monokai Extended";
        Ok(Config {
            input_file: input_file.map(|s| s.to_string()),
            input_format,
            output_file: output_file.map(|s| s.to_string()),
            output_format,
            color_enabled: color_enabled,
            theme_name: theme_name.to_string(),
        })
    }
}

fn build_clap_app(color_enabled: bool) -> clap::App<'static, 'static> {
    let color_setting = if color_enabled {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    ClapApp::new(crate_name!())
        .about("reformat between JSON, YAML and TOML.")
        .author(crate_authors!())
        .version(crate_version!())
        .global_setting(color_setting)
        .arg(
            Arg::with_name("INPUT_FILE")
                .help("set the input file to use. Assume STDIN if omitted")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("INPUT_FORMAT")
                .help("set the name of input format. Assume format by file extension if omitted.")
                .long("input-format")
                .takes_value(true)
                .value_name("FORMAT_NAME")
                .case_insensitive(true)
                .possible_values(&FileFormat::names()),
        )
        .arg(
            Arg::with_name("OUTPUT_FILE")
                .help("set the output file to use. Assume STDOUT if omitted")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("OUTPUT_FORMAT")
                .help("set the name of output format. Assume format by file extension if omitted")
                .long("output-format")
                .takes_value(true)
                .value_name("FORMAT_NAME")
                .case_insensitive(true)
                .possible_values(&FileFormat::names()),
        )
}

pub struct App {
    config: Config,
    assets: HighlightAssets,
}

impl App {
    pub fn new() -> Result<App, errors::Error> {
        let color_enabled = atty::is(atty::Stream::Stdout);
        let config = Config::new(build_clap_app(color_enabled), color_enabled)?;
        let assets = App::load_integrated_assets();

        debug!("config: {:?}", config);
        debug!(
            "syntaxes: {:?}",
            assets
                .syntaxes()
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>()
        );
        debug!(
            "themes: {:?}",
            assets
                .themes()
                .iter()
                .map(|&t| t
                    .name
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("** unnamed theme **"))
                .collect::<Vec<_>>()
        );
        Ok(App { config, assets })
    }

    pub fn run(&self) -> Result<(), errors::Error> {
        let input_text = self.read_from_input()?;
        let output_text = input_text.convert_to(self.config.output_format)?;

        self.write_to_output(&output_text)
    }

    fn load_integrated_assets() -> HighlightAssets {
        HighlightAssets::new(
            from_binary(include_bytes!("../../../assets/syntaxes.bin")),
            from_binary(include_bytes!("../../../assets/themes.bin")),
        )
    }

    fn read_from_input(&self) -> Result<FormattedText, errors::Error> {
        // open reader
        let stdin = stdin();
        let lock = stdin.lock();
        let mut reader = if let Some(f) = self.config.input_file.as_ref() {
            Box::new(BufReader::new(File::open(f)?)) as Box<dyn BufRead>
        } else {
            Box::new(lock) as Box<dyn BufRead>
        };

        // read
        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        Ok(FormattedText::new(self.config.input_format, text))
    }

    fn write_to_output(&self, text: &FormattedText) -> Result<(), errors::Error> {
        // open writer
        let stdout = stdout();
        let lock = stdout.lock();
        let mut w = if let Some(f) = self.config.output_file.as_ref() {
            Box::new(BufWriter::new(File::create(f)?)) as Box<dyn Write>
        } else {
            Box::new(lock) as Box<dyn Write>
        };

        // select printer
        let printer = if self.config.output_file.is_none() && self.config.color_enabled {
            Box::new(HighlightTextPrinter::new(&self.assets)) as Box<dyn Printer>
        } else {
            Box::new(PlainTextPrinter::default()) as Box<dyn Printer>
        };

        // print
        printer.print(&mut w, text)
    }
}

#[cfg(test)]
mod tests {
    use syntect::dumps::from_reader;
    use syntect::highlighting::ThemeSet;
    use syntect::parsing::SyntaxSet;

    #[test]
    fn syntax_set_asset() -> anyhow::Result<()> {
        let bytes: &[u8] = include_bytes!("../../../assets/syntaxes.bin");
        let _syntaxes: SyntaxSet = from_reader(bytes)?;
        Ok(())
    }

    #[test]
    fn theme_set_asset() -> anyhow::Result<()> {
        let bytes: &[u8] = include_bytes!("../../../assets/themes.bin");
        let _themes: ThemeSet = from_reader(bytes)?;
        Ok(())
    }
}
