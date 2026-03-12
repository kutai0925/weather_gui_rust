/******************************************************************************
 * Project: weather_gui
 * File: data.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/










use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub wind: Wind,
    pub clouds: Clouds,
    pub name: String, 
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64,
    pub humidity: u64,
}

#[derive(Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Deserialize)]
pub struct Clouds {
    pub all: u64,
}


pub struct WeatherData {
    pub temp_k: f64,
    pub humidity: u64,
    pub wind_speed: f64,
    pub clouds: u64,
    pub description: String,
    pub city_from_api: String, 
}