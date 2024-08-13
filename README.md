# A Weather App in the terminal

This is a simple weather app that runs in the terminal. It uses the OpenWeatherMap API to get the weather data and Ratatui to display the data in the terminal.

## Installation

1. Download the Exe in Releases
2. Put the Exe in a folder
3. Create a file called `secrets.json` in the same folder as the Exe
4. Add the following to the `secrets.json` file:
    ```json
    {
    "api_key": "YOUR_OPENWEATHERMAP_API_KEY"
    }
    ```
5. Run the Exe
6. Enjoy the weather!

## Usage

![Screenshot](https://i.imgur.com/e22Htoa.png)

The app has an Input field where you can enter the name of a city. After you press Enter, the app will display the weather data for that city.

## Features

- Display the current weather data for a city
- Display a 5 day forecast for a city in 3 hour intervals
- Search for a city by name