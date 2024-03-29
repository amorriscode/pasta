use arboard::Clipboard;
use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use db::ClipboardItem;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use sqlx::sqlite::SqlitePool;
use std::{env, thread::sleep, time::Duration};

mod commands;
mod db;
mod errors;
mod tui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    errors::install_hooks()?;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let matches = commands::cli().get_matches();

    match matches.subcommand() {
        Some(("start", _)) => loop {
            let mut clipboard = Clipboard::new().unwrap();
            let current_clipped_text = clipboard.get_text().unwrap();

            let latest_item = db::get_latest_clipboard_item(&pool).await?;

            match latest_item {
                None => {
                    db::create_clipboard_item(&pool, current_clipped_text).await?;
                }
                Some(pb_item) => {
                    if pb_item.content != current_clipped_text {
                        db::create_clipboard_item(&pool, current_clipped_text).await?;
                    }
                }
            }

            sleep(Duration::from_millis(500));
        },
        _ => {
            let mut terminal = tui::init()?;
            App::default().run(&mut terminal).await?;
            tui::restore()?;
        }
    }

    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    items: Vec<ClipboardItem>,
    exit: bool,
}

impl App {
    pub async fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

        while !self.exit {
            let pb_items = db::get_clipboard_items(&pool).await?;
            self.items = pb_items;

            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" pasta ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<q> ".blue().bold()]));
        let block = Block::default()
            .title(title)
            .title(instructions.position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let lines = self
            .items
            .iter()
            .map(|item| Line::from(item.clone().content.yellow()))
            .collect::<Vec<Line>>();

        let counter_text = Text::from(lines);

        Paragraph::new(counter_text).block(block).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() -> Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert_eq!(app.exit, true);

        Ok(())
    }
}
