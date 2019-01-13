use std::process;

use failure::Fail;

use refmt::errors;

mod app;
mod printer;

fn handle_error(error: &errors::Error) {
    use ansi_term::Color::Red;
    let label = Red.paint("[refmt error]");
    if let Some(cause) = error.cause() {
        eprintln!("{}: {}. cause: {}", label, error, cause);
    } else {
        eprintln!("{}: {}.", label, error);
    }
}

fn initialize() {
    env_logger::init();
}

fn terminate(code: i32) {
    process::exit(code)
}

fn run() -> Result<(), errors::Error> {
    let app = app::App::new()?;
    app.run()
}

fn main() {
    initialize();

    let code = match run() {
        Ok(()) => 0,
        Err(e) => {
            handle_error(&e);
            1
        }
    };

    terminate(code);
}
