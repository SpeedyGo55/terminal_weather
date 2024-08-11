// A Weather App in the terminal
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEvent},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::{Paragraph, Block, Borders},
    Terminal,
    prelude::*,
};

use std::io::{stdout, Result, Write};
use ratatui::crossterm::event::Event;
use tui_input::backend::crossterm as backend;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use ratatui::widgets::Wrap;

fn main() -> Result<()> {

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let stdout_var = stdout();
    let mut stdout_var = stdout_var.lock();
    terminal.clear()?;

    let mut input: Input = "Basel".into();
    stdout_var.flush()?;

    loop {
        let event = event::read()?;

        if let Event::Key(KeyEvent { code, ..}) = event {
            match code {
                KeyCode::Esc => break,
                KeyCode::Enter => {},
                _ => {
                    input.handle_event(&event);
                }
            }
        }
        terminal.draw(|frame| {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(1),
                    ]
                        .as_ref(),
                )
                .split(frame.size());

            frame.render_widget(
                Paragraph::new("Weather App")
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .alignment(Alignment::Center),
                outer_layout[0],
            );

            frame.render_widget(
                Paragraph::new("Press Esc to exit")
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .alignment(Alignment::Left),
                outer_layout[3],
            );

            frame.render_widget(
                Paragraph::new("Weather Information")
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .alignment(Alignment::Center),
                outer_layout[2],
            );

            frame.render_widget(
                Paragraph::new(input.value())
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Search"))
                    .alignment(Alignment::Center),
                outer_layout[1],
            );
        })?;
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}