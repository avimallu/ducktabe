use crate::tui::app::App;
use crate::tui::ui::ui;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::Terminal;
use std::{error::Error, io};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => return Ok(true),
                KeyEvent {
                    code:
                        KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Char('k')
                        | KeyCode::Char('j')
                        | KeyCode::Char('h')
                        | KeyCode::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => app.update_scroll(key.code),
                KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('R'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let new_query = app.get_user_query();
                    app.update_query(Some(&new_query));
                    app.update_output();
                }
                KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('W'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    app.wrap_output = !app.wrap_output;
                    app.update_output();
                }
                _ => {
                    app.query_editor.input(key);
                }
            }
        }
    }
}
