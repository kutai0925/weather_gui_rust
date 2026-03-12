/******************************************************************************
 * Project: weather_gui
 * File: main.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/





mod app;
mod get_data;
mod data;
mod config;

use app::WeatherApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rust Weather App",
        options,
        Box::new(|_cc| Ok(Box::new(WeatherApp::default()))),
    )
}