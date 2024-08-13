// A Weather App in the terminal using Ratatui and Tui-Inputy
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEvent},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Paragraph, Block, Borders},
    Terminal,
    prelude::*,
};

use std::io::{stdout, Result, Write};
use std::string;
use ratatui::crossterm::event::Event;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};

use json;

use reqwest::Response;

use itertools::Itertools;
use json::stringify;

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

    let temp_x_axis = Axis::default()
        .title("Time [D]")
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 40.0])
        .labels(["0", "1", "2", "3", "4", "5"]); 

    let temp_y_axis = Axis::default()
        .title("Temperature [C°]")
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 50.0])
        .labels(["0", "5", "10", "15", "20", "25", "30", "35", "40", "45", "50"]);

    let rain_x_axis = Axis::default()
        .title("Time [D]")
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 40.0])
        .labels(["0", "1", "2", "3", "4", "5"]);

    let rain_y_axis = Axis::default()
        .title("Rain [mm]")
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 10.0])
        .labels(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);
    
    let mut input: Input = "".into();
    stdout_var.flush()?;

    loop {
        let event = event::read()?;

        if let Event::Key(KeyEvent { code, ..}) = event {
            match code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    response = reqwest::get(&format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", input.value(), api_key)).await.unwrap();
                    let next_weather_data = json::parse(response.text().await.unwrap().as_str()).unwrap();
                    match next_weather_data["main"]["temp"] {
                        json::JsonValue::Null => {},
                        _ => {
                            current_weather_data = next_weather_data;
                        }
                    }
                    response = reqwest::get(&format!("https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}", input.value(), api_key)).await.unwrap();
                    let next_forecast_data = json::parse(response.text().await.unwrap().as_str()).unwrap();
                    match next_forecast_data["list"].len() {
                        0 => {},
                        _ => {
                            forecast_data = next_forecast_data;
                        }
                    }
                    input.reset();
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
                        Constraint::Length(3), // Header
                        Constraint::Length(3), // Search
                        Constraint::Length(3), // Weather Info
                        Constraint::Min(0), // Forecast
                        Constraint::Length(1), // Footer
                    ].as_ref(),
                )
                .split(frame.area());

            let header_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Fill(1), // Temperature Big
                        Constraint::Fill(1), // City
                        Constraint::Fill(1) // Weather Icon
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
                        Constraint::Percentage(50), // Temperature over 5 days as a graph
                        Constraint::Percentage(50), // Rain over 5 days as a graph
                    ].as_ref(),
                )
                .split(outer_layout[3]);
            
            // the forecast data is in 3 hour intervals. We want to get the average temperature for each day, structured as a list of tuples (pos on x, temperature): &[(f64, f64)]
            
            let mut forecast_temp_vec: Vec<(f64, f64)> = Vec::new();
            
            for i in 0..forecast_data["list"].len() {
                let temp = forecast_data["list"][i]["main"]["temp"].as_f64().unwrap()-273.15;
                forecast_temp_vec.push((i as f64, temp));
            }
            
            let forecast_temp_list: &mut [(f64, f64); 40] = &mut [(0.0, 0.0); 40];
            
            forecast_temp_list.iter_mut().set_from(forecast_temp_vec.iter().cloned());
            
            let forecast_temp_dataset = Dataset::default()
                .name("Temperature")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Yellow))
                .data(forecast_temp_list);
            
            let temp_chart = Chart::new(Vec::from([forecast_temp_dataset]))
                .block(
                    Block::default()
                        .title("Temperature over 5 days")
                        .title_style(Style::default().fg(Color::White))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White)),
                )
                .x_axis(temp_x_axis.clone())
                .y_axis(temp_y_axis.clone())
                .style(Style::default().fg(Color::White));
            
            let mut forecast_rain_vec: Vec<(f64, f64)> = Vec::new();

            for i in 0..forecast_data["list"].len() {
                let rain = forecast_data["list"][i]["rain"]["3h"].as_f64().unwrap_or(0.0);
                forecast_rain_vec.push((i as f64, rain));
            }
            
            let forecast_rain_list: &mut [(f64, f64); 40] = &mut [(0.0, 0.0); 40];
            
            forecast_rain_list.iter_mut().set_from(forecast_rain_vec.iter().cloned());
            
            let forecast_rain_dataset = Dataset::default()
                .name("Rain")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Bar)
                .style(Style::default().fg(Color::Blue))
                .data(forecast_rain_list);
            
            let rain_chart = Chart::new(Vec::from([forecast_rain_dataset]))
                .block(
                    Block::default()
                        .title("Rain over 5 days")
                        .title_style(Style::default().fg(Color::White))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White)),
                )
                .x_axis(rain_x_axis.clone())
                .y_axis(rain_y_axis.clone())
                .style(Style::default().fg(Color::White));
            
            

            frame.render_widget(
                Paragraph::new(format!("{:.1} C°", current_weather_data["main"]["temp"].as_f64().unwrap()-273.15))
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
                Paragraph::new(format!("Temperature: {:.1} C° Feels like: {:.1} C°", current_weather_data["main"]["temp"].as_f64().unwrap()-273.15, current_weather_data["main"]["feels_like"].as_f64().unwrap()-273.15))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Temperature"))
                    .alignment(Alignment::Center),
                weather_info_layout[0],
            );
            
            frame.render_widget(
                Paragraph::new(format!("Humidity: {}%", current_weather_data["main"]["humidity"].as_i64().unwrap()))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Humidity"))
                    .alignment(Alignment::Center),
                weather_info_layout[1],
            );
            
            frame.render_widget(
                Paragraph::new(format!("Rain: {}mm", current_weather_data["rain"]["1h"].as_f64().unwrap_or(0.0)))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Rain"))
                    .alignment(Alignment::Center),
                weather_info_layout[2],
            );
            
            frame.render_widget(
                Paragraph::new(format!("Wind Speed: {}m/s Direction: {}°", current_weather_data["wind"]["speed"].as_f64().unwrap_or(0.0), current_weather_data["wind"]["deg"].as_f64().unwrap_or(0.0)))
                    .style(Style::default().fg(Color::White).bg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Wind Speed"))
                    .alignment(Alignment::Center),
                weather_info_layout[3],
            );
            
            
            frame.render_widget(
                temp_chart,
                forecast_layout[0],
            );
            
            frame.render_widget(
                rain_chart,
                forecast_layout[1],
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