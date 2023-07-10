use std::collections::HashMap;

use serde::Deserialize;

use super::api::Api;

// Beginning of Current Weather Structs

#[derive(Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(default = "default_integer")]
    pub sea_level: i64,
    #[serde(default = "default_integer")]
    pub grnd_level: i64,
}

fn default_integer() -> i64 {
    0
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    #[serde(default = "default_float")]
    pub gust: f64,
}

fn default_float() -> f64 {
    0.0
}

#[derive(Deserialize)]
pub struct Rain {
    #[serde(alias = "1h", default = "default_float")]
    pub one_hour: f64,
    #[serde(alias = "3h", default = "default_float")]
    pub three_hour: f64,
}

#[derive(Deserialize)]
pub struct Snow {
    #[serde(alias = "1h", default = "default_float")]
    pub one_hour: f64,
    #[serde(alias = "3h", default = "default_float")]
    pub three_hour: f64,
}

fn default_rain() -> Rain {
    Rain {
        one_hour: 0.0,
        three_hour: 0.0,
    }
}

fn default_snow() -> Snow {
    Snow {
        one_hour: 0.0,
        three_hour: 0.0,
    }
}

#[derive(Deserialize)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Deserialize)]
pub struct System {
    #[serde(alias = "type")]
    pub ty: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize)]
pub struct CurrentWeather {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    #[serde(default = "default_rain")]
    pub rain: Rain,
    #[serde(default = "default_snow")]
    pub snow: Snow,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: System,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

// End of Current Weather structs

#[derive(Deserialize)]
pub struct ForecastMain {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(default = "default_integer")]
    pub sea_level: i64,
    #[serde(default = "default_integer")]
    pub grnd_level: i64,
    pub temp_kf: f64,
}

#[derive(Deserialize)]
pub struct ForecastSystem {
    pub pod: String,
}

#[derive(Deserialize)]
pub struct ForecastWeather {
    pub dt: i64,
    pub main: ForecastMain,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: i64,
    pub pop: f64,
    #[serde(default = "default_rain")]
    pub rain: Rain,
    #[serde(default = "default_snow")]
    pub snow: Snow,
    pub sys: ForecastSystem,
    pub dt_txt: String,
}

#[derive(Deserialize)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub coord: Coordinates,
    pub country: String,
    pub population: i64,
    pub timezone: i64,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub cod: String,
    pub message: i64,
    pub cnt: i64,
    pub list: Vec<ForecastWeather>,
    pub city: City,
}

pub struct OpenWeatherMapService {
    weather_api: Api,
    forecast_api: Api,
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
        let forecast_api = Api::new(
            url.clone(),
            String::from("/data/2.5/forecast"),
            default_parameters.clone(),
        );
        OpenWeatherMapService {
            weather_api,
            forecast_api,
        }
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

    pub fn request_forecast(&self, zip_code: String, country_code: String) -> Forecast {
        let mut zip_parameter = zip_code;
        zip_parameter.push_str(",");
        zip_parameter.push_str(&country_code);
        let parameters: HashMap<String, String> = HashMap::from([
            (String::from("zip"), zip_parameter),
            (String::from("units"), String::from("imperial")),
        ]);
        let url = self.forecast_api.construct_request_url(parameters);
        let response = reqwest::blocking::get(url);
        let result = match response {
            Ok(result) => result,
            Err(error) => panic!("Error retreiving data: {}", error),
        };
        let data: Forecast = match result.json() {
            Ok(text) => text,
            Err(error) => panic!("Error parsing text: {}", error),
        };
        data
    }
}
