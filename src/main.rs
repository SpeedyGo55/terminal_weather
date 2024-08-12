use std::fmt::format;
// A Weather App in the terminal using Ratatui and Tui-Inputy
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

use json;
use json::JsonResult;

use reqwest::{Error, Response};

#[tokio::main]
async fn main() -> Result<()> {

    let secrets = json::parse(include_str!("../secrets.json")).unwrap();
    let api_key: &str = secrets["API_KEY"].as_str().unwrap();

    let mut response: Response = reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?q=Basel&appid={}", api_key)).await.unwrap();
    let mut current_weather_data = json::parse(response.text().await.unwrap().as_str()).unwrap();
    response = reqwest::get(format!("https://api.openweathermap.org/data/2.5/forecast?q=Basel&appid={}", api_key)).await.unwrap();
    let mut forecast_data = json::parse(response.text().await.unwrap().as_str()).unwrap();

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
                KeyCode::Enter => {
                    response = reqwest::get(&format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", input.value(), api_key)).await.unwrap();
                    current_weather_data = json::parse(response.text().await.unwrap().as_str()).unwrap();
                    response = reqwest::get(&format!("https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}", input.value(), api_key)).await.unwrap();
                    forecast_data = json::parse(response.text().await.unwrap().as_str()).unwrap();
                },
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
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(1),
                    ].as_ref(),
                )
                .split(frame.size());

            let header_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(33), // Temperature Big
                        Constraint::Percentage(33), // City
                        Constraint::Percentage(33) // Weather Icon
                    ].as_ref(),
                )
                .split(outer_layout[0]);

            let weather_info_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(25), // Temperature
                        Constraint::Percentage(25), // Humidity
                        Constraint::Percentage(25), // Rain
                        Constraint::Percentage(25), // Wind Speed
                    ].as_ref(),
                )
                .split(outer_layout[2]);

            let forecast_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(50), // Temperature over 5 days
                        Constraint::Percentage(50), // Rain over 5 days
                    ].as_ref(),
                )
                .split(outer_layout[3]);

            frame.render_widget(
                Paragraph::new(format!("{:.2} C°", current_weather_data["main"]["temp"].as_f64().unwrap()-273.15))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Weather"))
                    .alignment(Alignment::Center),
                header_layout[2],
            );

            frame.render_widget(
                Paragraph::new(format!("{}", current_weather_data["name"].as_str().unwrap()))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("City"))
                    .alignment(Alignment::Center),
                header_layout[1],
            );

            frame.render_widget(
                Paragraph::new(format!("{}", current_weather_data["weather"][0]["description"].as_str().unwrap()))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Weather"))
                    .alignment(Alignment::Center),
                header_layout[0],
            );

            frame.render_widget(
                Paragraph::new(input.value())
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Search"))
                    .alignment(Alignment::Center),
                outer_layout[1],
            );

            frame.render_widget(
                Paragraph::new("Press Esc to exit")
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .alignment(Alignment::Left),
                outer_layout[4],
            );
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}