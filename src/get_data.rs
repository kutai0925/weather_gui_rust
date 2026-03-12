/******************************************************************************
 * Project: weather_gui
 * File: get_data.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/








use crate::config::OPENWEATHER_API_KEY;
use crate::data::{WeatherData, WeatherResponse};
use anyhow::{anyhow, Result};

pub fn fetch_weather(city: &str) -> Result<WeatherData> {
    let city = city.trim();
    if city.is_empty() {
        return Err(anyhow!("City is empty."));
    }

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city,
        OPENWEATHER_API_KEY
    );

    let response: WeatherResponse = reqwest::blocking::get(url)?
        .error_for_status()?
        .json()?;

   Ok(WeatherData {
    temp_k: response.main.temp,
    humidity: response.main.humidity,
    wind_speed: response.wind.speed,
    clouds: response.clouds.all,
    description: response
        .weather
        .get(0)
        .map(|w| w.description.clone())
        .unwrap_or_else(|| "n/a".to_string()),
    city_from_api: response.name,
})
}