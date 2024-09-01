mod db;
mod ui;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    let mut state = ui::State::default();

    let mut client: Box<dyn db::Queryable> =
        Box::new(db::postgres::Client::new("host=localhost user=postgres")?);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|frame| state.ui(frame))?;
        state.handle_event();

        if state.should_quit() {
            break;
        }

        loop {
            match state.pop_query() {
                Some(q) => {
                    state.add_results(client.query(q, Vec::new())?);
                }
                None => break,
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
