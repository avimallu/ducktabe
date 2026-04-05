mod ducky;
mod tui;
use std::path::PathBuf;
use std::{error::Error, io};

use clap::Parser;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

#[derive(Parser, Debug)]
#[command(version = "pre-alpha")]
#[command(about = "DuckTabE")]
#[command(long_about = "DuckDB-based Tabular Explorer")]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Ratatui boilerplate
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = tui::app::App::new(args.file);
    let res = tui::events::run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        // if do_print {
        //     // app.print_json()?;

        // }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
