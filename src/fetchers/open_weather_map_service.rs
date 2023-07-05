use std::collections::HashMap;

use serde::Deserialize;

use super::api::Api;

#[derive(Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: i64,
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Deserialize)]
pub struct CurrentWeather {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub name: String,
}

pub struct OpenWeatherMapService {
    weather_api: Api,
}

impl OpenWeatherMapService {
    pub fn new(url: String, api_key: String) -> Self {
        let default_parameters: HashMap<String, String> =
            HashMap::from([(String::from("appid"), api_key)]);
        let weather_api = Api::new(
            url.clone(),
            String::from("/data/2.5/weather"),
            default_parameters.clone(),
        );
        OpenWeatherMapService { weather_api }
    }

    pub fn request_weather(&self, zip_code: String, country_code: String) -> CurrentWeather {
        let mut zip_parameter = zip_code;
        zip_parameter.push_str(",");
        zip_parameter.push_str(&country_code);
        let parameters: HashMap<String, String> = HashMap::from([
            (String::from("zip"), zip_parameter),
            (String::from("units"), String::from("imperial")),
        ]);
        let url = self.weather_api.construct_request_url(parameters);
        let response = reqwest::blocking::get(url);
        let result = match response {
            Ok(result) => result,
            Err(error) => panic!("Error retreiving data: {}", error),
        };
        let data: CurrentWeather = match result.json() {
            Ok(text) => text,
            Err(error) => panic!("Error parsing text: {}", error),
        };
        data
    }
}
