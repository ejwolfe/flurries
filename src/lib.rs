mod configuration;
mod fetchers;

use std::env::var;

use configuration::environment;
use fetchers::open_weather_map_service::{CurrentWeather, Forecast, OpenWeatherMapService};

pub fn setup_environment() -> OpenWeatherMapService {
    environment::setup_environment();
    let api_key = get_environment_variable(environment::API_KEY_KEY);
    let url = get_environment_variable(environment::WEATHER_API_URL_KEY);
    OpenWeatherMapService::new(url, api_key)
}

pub fn get_weather(service: &OpenWeatherMapService) -> CurrentWeather {
    let zip_code = get_environment_variable(environment::ZIP_CODE_KEY);
    let country_code = get_environment_variable(environment::COUNTRY_CODE_KEY);
    service.request_weather(zip_code, country_code)
}

pub fn get_forecast(service: &OpenWeatherMapService) -> Forecast {
    let zip_code = get_environment_variable(environment::ZIP_CODE_KEY);
    let country_code = get_environment_variable(environment::COUNTRY_CODE_KEY);
    service.request_forecast(zip_code, country_code)
}

fn get_environment_variable(key: &str) -> String {
    let variable_result = var(key);
    if let Ok(variable) = variable_result {
        variable
    } else {
        panic!("Environment variable is missing");
    }
}
