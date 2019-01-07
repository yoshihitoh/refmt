use std::fs::File;
use std::io::{self, stdin, stdout, BufReader, BufWriter, Read, Stdin, Stdout, Write};
use std::path::Path;
use std::str::FromStr;

use clap::{crate_authors, crate_name, crate_version, App, Arg};
use syntect::easy::HighlightLines;
use syntect::util::as_24_bit_terminal_escaped;

use refmt_core::assets::HighlightAssets;
use refmt_core::translator::json::JsonTranslator;
use refmt_core::translator::toml::TomlTranslator;
use refmt_core::translator::translator::{Format, TranslateError, Translator};
use refmt_core::translator::yaml::YamlTranslator;

#[derive(Debug)]
enum ConvertError {
    IoError(io::Error),
    TranslateError(TranslateError),
    ArgumentError(String),
    HighlightLoadingError(syntect::LoadingError),
    BincodeError(bincode::Error),
}

impl From<io::Error> for ConvertError {
    fn from(e: io::Error) -> Self {
        ConvertError::IoError(e)
    }
}

impl From<TranslateError> for ConvertError {
    fn from(e: TranslateError) -> Self {
        ConvertError::TranslateError(e)
    }
}

impl From<syntect::LoadingError> for ConvertError {
    fn from(e: syntect::LoadingError) -> Self {
        ConvertError::HighlightLoadingError(e)
    }
}

impl From<bincode::Error> for ConvertError {
    fn from(e: bincode::Error) -> Self {
        ConvertError::BincodeError(e)
    }
}

type ConvertResult<T> = Result<T, ConvertError>;

static JSON_TRANSLATOR: JsonTranslator = JsonTranslator {};
static TOML_TRANSLATOR: TomlTranslator = TomlTranslator {};
static YAML_TRANSLATOR: YamlTranslator = YamlTranslator {};

fn translator_for(format: Format) -> &'static Translator {
    match format {
        Format::Json => &JSON_TRANSLATOR,
        Format::Toml => &TOML_TRANSLATOR,
        Format::Yaml => &YAML_TRANSLATOR,
    }
}

#[derive(Debug)]
struct ProgramOptions {
    input: Option<String>,
    input_format: Format,
    output: Option<String>,
    output_format: Format,
}

fn infer_format(file: Option<&str>, format_name: Option<&str>) -> ConvertResult<Format> {
    if file.is_none() && format_name.is_none() {
        return Err(ConvertError::ArgumentError(
            "cannot determine file format, need to specify either FILE or FORMAT".to_string(),
        ));
    }

    let format_name = format_name
        .or(file.and_then(|f| Path::new(f).extension().map(|ext| ext.to_str().unwrap())));
    Ok(Format::from_str(format_name.unwrap_or(""))?)
}

fn parse_args() -> ConvertResult<ProgramOptions> {
    let mut app = App::new(crate_name!())
        .about("Translate data format into another one.")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("INPUT")
                .help("set the input file to use")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("INPUT_FORMAT")
                .help("set the name of input format")
                .long("input-format")
                .takes_value(true)
                .value_name("FORMAT_NAME")
                .case_insensitive(true)
                .possible_values(&Format::names()),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("set the output file to use")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("OUTPUT_FORMAT")
                .help("set the name of output format")
                .long("output-format")
                .takes_value(true)
                .value_name("FORMAT_NAME")
                .case_insensitive(true)
                .possible_values(&Format::names()),
        );
    let m = app.clone().get_matches();

    let input_format =
        infer_format(m.value_of("INPUT"), m.value_of("INPUT_FORMAT")).map_err(|e| {
            app.print_long_help().unwrap_or(());

            ConvertError::ArgumentError(format!(
                "cannot determine format of the input file: {:?}",
                e
            ))
        })?;
    let output_format = infer_format(m.value_of("OUTPUT"), m.value_of("OUTPUT_FORMAT"));
    let output_format = if let Err(ConvertError::ArgumentError(_)) = output_format {
        input_format
    } else {
        output_format?
    };

    let option = ProgramOptions {
        input: m.value_of("INPUT").map(|s| s.to_string()),
        input_format,
        output: m.value_of("OUTPUT").map(|s| s.to_string()),
        output_format,
    };

    Ok(option)
}

fn main() -> ConvertResult<()> {
    let option = parse_args()?;
    run(option)
}

fn reader_for(file: Option<&str>, sin: Stdin) -> ConvertResult<Box<Read>> {
    let r = match file {
        Some(f) => Box::new(File::open(f)?) as Box<Read>,
        None => Box::new(sin) as Box<Read>,
    };
    Ok(r)
}

fn writer_for(file: Option<&str>, sout: Stdout) -> ConvertResult<Box<Write>> {
    let w = match file {
        Some(f) => Box::new(File::create(f)?) as Box<Write>,
        None => Box::new(sout) as Box<Write>,
    };
    Ok(w)
}

fn read_all_text(r: Box<Read>) -> ConvertResult<String> {
    let mut reader = BufReader::new(r);
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}

fn write_all_text(
    w: Box<Write>,
    s: &str,
    fmt: Format,
    assets: &HighlightAssets,
) -> ConvertResult<()> {
    let mut writer = BufWriter::new(w);

    let ps = &assets.syntax_set;
    let syntax = ps.find_syntax_by_extension(fmt.preferred_extension());
    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty && syntax.is_some() {
        let syntax = syntax.unwrap();
        let mut h = HighlightLines::new(syntax, &assets.theme_set.themes["Monokai Extended"]);

        let ranges = h.highlight(s, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges, true);
        writeln!(writer, "{}", escaped)?;
    } else {
        writeln!(writer, "{}", s)?;
    }

    Ok(())
}

fn run(option: ProgramOptions) -> ConvertResult<()> {
    let assets = HighlightAssets::load_integrated()?;

    let r = reader_for(option.input.as_ref().map(|s| s.as_str()), stdin())?;
    let w = writer_for(option.output.as_ref().map(|s| s.as_str()), stdout())?;

    let from_text = read_all_text(r)?;
    let translator = translator_for(option.output_format);
    let to_text = translator.translate(&from_text, option.input_format)?;
    write_all_text(w, &to_text, option.output_format, &assets)
}
