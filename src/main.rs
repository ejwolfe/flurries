use flurries::{get_forecast, get_weather, setup_environment};

fn main() {
    let service = setup_environment();
    let current_weather = get_weather(&service);
    let description = if current_weather.weather.len() > 0 {
        &current_weather.weather[0].description
    } else {
        ""
    };
    let icon = if current_weather.weather.len() > 0 {
        &current_weather.weather[0].icon
    } else {
        ""
    };
    println!("{}", &current_weather.name);
    println!("{}°", &current_weather.main.temp.round());
    println!("{} {}", print_weather_condition(icon), description);
    println!(
        "H:{}° L:{}°",
        &current_weather.main.temp_max.round(),
        &current_weather.main.temp_min.round()
    );
    let forecast = get_forecast(&service);
    println!("");
    println!("{}", &forecast.city.name);
    println!("{}", &forecast.list[0].pop);
}

fn print_weather_condition(icon: &str) -> &str {
    match icon {
        "01d" => "☀️",
        "02d" => "⛅",
        "03d" => "☁️",
        "04d" => "☁️",
        "09d" => "🌧️",
        "10d" => "☔",
        "11d" => "🌩️",
        "13d" => "🌨️",
        "50d" => "🌫️",
        "01n" => "🌑",
        "02n" => "☁️",
        "03n" => "☁️",
        "04n" => "☁️",
        "09n" => "🌧️",
        "10n" => "☔",
        "11n" => "🌩️",
        "13n" => "🌨️",
        "50n" => "🌫️",
        _ => "",
    }
}
