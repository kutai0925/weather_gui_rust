/******************************************************************************
 * Project: weather_gui
 * File: app.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/






use crate::data::WeatherData;
use crate::get_data::fetch_weather;
use eframe::egui;

#[derive(Default)]
pub struct WeatherApp {
    pub city: String,
    pub result: Option<WeatherData>,
    pub error: String,
    pub unit: TemperatureUnit,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TemperatureUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        TemperatureUnit::Celsius
    }
}

impl TemperatureUnit {
    fn next(self) -> Self {
        match self {
            TemperatureUnit::Kelvin => TemperatureUnit::Celsius,
            TemperatureUnit::Celsius => TemperatureUnit::Fahrenheit,
            TemperatureUnit::Fahrenheit => TemperatureUnit::Kelvin,
        }
    }

    fn format_temp(self, k: f64) -> String {
        match self {
            TemperatureUnit::Kelvin => format!("{:.0} K", k), // ohne °
            TemperatureUnit::Celsius => format!("{:.0} °C", k - 273.15),
            TemperatureUnit::Fahrenheit => format!("{:.0} °F", (k - 273.15) * 9.0 / 5.0 + 32.0),
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Weather App");
            ui.add_space(8.0);

            // City input (nur Stadt!)
            ui.horizontal(|ui| {
                ui.label("City:");
                ui.text_edit_singleline(&mut self.city);
            });

            ui.add_space(8.0);

            // Buttons direkt darunter
            ui.horizontal(|ui| {
                if ui.button("Fetch").clicked() {
                    match fetch_weather(&self.city) {
                        Ok(data) => {
                            self.result = Some(data);
                            self.error.clear();
                        }
                        Err(e) => {
                            self.result = None;
                            self.error = e.to_string();
                        }
                    }
                }

                if ui.button("Switch Unit").clicked() {
                    self.unit = self.unit.next();
                }
            });

            if !self.error.is_empty() {
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::RED, &self.error);
            }

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(14.0);

            // OUTPUT Bereich (zentriert)
            if let Some(data) = &self.result {
                ui.vertical_centered(|ui| {
                    // Temperatur groß in der Mitte
                    let temp_text = self.unit.format_temp(data.temp_k);
                    ui.label(egui::RichText::new(temp_text).size(42.0));

                    // Stadtname direkt darunter (aus Eingabefeld)
                    // (du wolltest den Stadt-Namen aus dem Textfeld – wir nutzen self.city)
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new(data.city_from_api.as_str()).size(18.0));

                    ui.add_space(12.0);

                    // "space-evenly": 3 Werte nebeneinander
                    // Feuchtigkeit, Wind, Himmel-Klarheit (Clouds %)
                    // -> Cloudiness % = Wolkenanteil. (Je kleiner, desto "klarer")
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width(), 0.0),
                        egui::Layout::left_to_right(egui::Align::Center)
                            .with_main_justify(true)
                            .with_cross_align(egui::Align::Center),
                        |ui| {
                            metric_block(ui, "Humidity", &format!("{} %", data.humidity));
                            metric_block(ui, "Wind", &format!("{:.1} m/s", data.wind_speed));
                            metric_block(ui, "Clouds", &format!("{} %", data.clouds));
                        },
                    );

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(format!("Sky: {}", data.description)).size(14.0));
                });
            } else {
                ui.label("Enter a city and click Fetch.");
                ui.small("API key must be set globally via OPENWEATHER_API_KEY.");
            }
        });
    }
}

fn metric_block(ui: &mut egui::Ui, title: &str, value: &str) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(title).size(13.0).strong());
        ui.label(egui::RichText::new(value).size(16.0));
    });
}






