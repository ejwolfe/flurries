use std::{env::set_var, fs, str::Split};

pub const API_KEY_KEY: &str = "API_KEY";
pub const ZIP_CODE_KEY: &str = "ZIP_CODE";
pub const COUNTRY_CODE_KEY: &str = "COUNTRY_CODE";
pub const WEATHER_API_URL_KEY: &str = "WEATHER_API_URL";

pub fn setup_environment() {
    let file_path = ".env";
    let file = fs::read_to_string(file_path);
    if let Ok(contents) = file {
        parse_env_file(contents);
    } else {
        panic!("Environment file does not exist!");
    }
}

fn parse_env_file(data: String) {
    let lines = data.lines();
    for line in lines {
        let mut value = line.split("=");
        let key = get_next_value(&mut value);
        let value = get_next_value(&mut value);
        set_var(key, value);
    }
}

fn get_next_value(split: &mut Split<&str>) -> String {
    if let Some(value) = split.next() {
        value.to_string()
    } else {
        panic!("There is no next value")
    }
}
